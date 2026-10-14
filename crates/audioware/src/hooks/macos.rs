//! macOS hook modes (stage 3 of the Audioware port).
//!
//! One binary, three behaviours. The mode is one word in `mac-hooks.txt`, next to the plugin dylib
//! (`red4ext/plugins/audioware/mac-hooks.txt`):
//!
//! - `off`: no hook is attached. This is also the mode when the file is missing, unreadable or says something else.
//! - `probe`: every hook is attached with a detour that counts its calls, logs the first [LOGGED_CALLS] of them (arguments and result as raw
//!   hexadecimal values, nothing is dereferenced), calls the original with the very arguments it got and returns the original's result
//!   unchanged. A running count of every hook is logged every 60 seconds.
//! - `on`: the real detours.
//!
//! Why three: the Mac targets of the hooks were matched statically against the Windows functions (B1 report 2.1), no hook of this plugin has
//! run in the game yet, and the private loader accepts an `Attach` on an offset that is not on its allowlist without doing anything. `off`
//! shows that the plugin loads, registers its natives and starts its audio engine with no hook at all. `probe` shows that each target is
//! reached with plausible arguments and that the pass-through is right. `on` runs the plugin.

use std::{
    ffi::{CStr, OsStr, c_char, c_int, c_void},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::{
        Condvar, Mutex, MutexGuard, OnceLock, PoisonError,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

use red4ext_rs::{PluginOps, SdkEnv, log};

use crate::Audioware;

/// Name of the mode file, in the folder of the plugin dylib.
pub const MODE_FILE: &str = "mac-hooks.txt";
/// How many calls of each hook a probe logs in full.
pub const LOGGED_CALLS: u64 = 3;
/// How often a probe run logs the running counts.
const REPORT_EVERY: Duration = Duration::from_secs(60);

/// What the hooks do, see the module documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Off,
    Probe,
    On,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Off => "off",
            Mode::Probe => "probe",
            Mode::On => "on",
        }
    }

    /// The first line of the file that is not empty and not a `#` comment, one word, any case.
    fn parse(text: &str) -> Option<Self> {
        let word = text
            .lines()
            .map(str::trim)
            .find(|x| !x.is_empty() && !x.starts_with('#'))?;
        match word.to_ascii_lowercase().as_str() {
            "off" => Some(Mode::Off),
            "probe" => Some(Mode::Probe),
            "on" => Some(Mode::On),
            _ => None,
        }
    }
}

static MODE: OnceLock<Mode> = OnceLock::new();

/// The hook mode, read once from [MODE_FILE]. Needs the plugin to be loaded (it logs through the SDK).
pub fn mode() -> Mode {
    *MODE.get_or_init(read_mode)
}

fn read_mode() -> Mode {
    let env = Audioware::env();
    let Some(dir) = plugin_dir() else {
        log::warn!(env, "Audioware hook mode: off (cannot find the folder of the plugin)");
        return Mode::Off;
    };
    let file = dir.join(MODE_FILE);
    match std::fs::read_to_string(&file) {
        Ok(text) => match Mode::parse(&text) {
            Some(mode) => {
                log::info!(env, "Audioware hook mode: {} (from {})", mode.as_str(), file.display());
                mode
            }
            None => {
                log::warn!(
                    env,
                    "Audioware hook mode: off ({} should say off, probe or on, it says {:?})",
                    file.display(),
                    text.trim()
                );
                Mode::Off
            }
        },
        Err(e) => {
            log::info!(env, "Audioware hook mode: off ({}: {e})", file.display());
            Mode::Off
        }
    }
}

#[repr(C)]
struct DlInfo {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

unsafe extern "C" {
    fn dladdr(addr: *const c_void, info: *mut DlInfo) -> c_int;
}

/// The folder of this dylib, from the path the loader opened it with.
fn plugin_dir() -> Option<PathBuf> {
    let mut info = DlInfo {
        dli_fname: std::ptr::null(),
        dli_fbase: std::ptr::null_mut(),
        dli_sname: std::ptr::null(),
        dli_saddr: std::ptr::null_mut(),
    };
    // SAFETY: `dladdr` reads the address and fills `info`; the address is a function of this image.
    let found = unsafe { dladdr(plugin_dir as *const c_void, &mut info) };
    if found == 0 || info.dli_fname.is_null() {
        return None;
    }
    // SAFETY: on success `dli_fname` is a NUL terminated path owned by the dynamic loader.
    let name = unsafe { CStr::from_ptr(info.dli_fname) };
    Path::new(OsStr::from_bytes(name.to_bytes()))
        .parent()
        .map(Path::to_path_buf)
}

fn locked<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The first bytes (at most 8) of any value as a number, for the probe logs. Nothing is dereferenced.
pub fn raw<T>(value: &T) -> u64 {
    let mut out = [0u8; 8];
    let len = std::mem::size_of::<T>().min(8);
    // SAFETY: `value` is a valid reference, `len` is at most its size and at most the size of `out`.
    unsafe { std::ptr::copy_nonoverlapping((value as *const T).cast::<u8>(), out.as_mut_ptr(), len) };
    u64::from_le_bytes(out)
}

/// The call counter of one probed hook.
pub struct Probe {
    name: &'static str,
    calls: AtomicU64,
}

static PROBES: Mutex<Vec<&'static Probe>> = Mutex::new(Vec::new());

impl Probe {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: AtomicU64::new(0),
        }
    }

    /// Adds the probe to the list the periodic report reads.
    pub fn register(&'static self) {
        locked(&PROBES).push(self);
    }

    /// Counts one call, returns its number (the first call is 1).
    pub fn enter(&self) -> u64 {
        self.calls.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Logs one of the first calls: the thread, the raw arguments and the raw result.
    pub fn log_call(&self, n: u64, args: &[u64], result: u64) {
        let args = args
            .iter()
            .map(|x| format!("{x:#x}"))
            .collect::<Vec<_>>()
            .join(", ");
        log::info!(
            Audioware::env(),
            "probe {}: call {n}/{LOGGED_CALLS} on {:?}, args [{args}], result {result:#x}",
            self.name,
            std::thread::current().id(),
        );
    }
}

static ATTACHED: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicUsize = AtomicUsize::new(0);
static SKIPPED: AtomicUsize = AtomicUsize::new(0);

/// Counts the result of one `Attach`. The private loader answers `true` for an offset that is not on its allowlist (and does nothing), so
/// `true` only means the call went through: the loader's own `[Hooking] Hook #n INSTALLED` lines are the proof of a patched target.
pub fn attached(name: &str, ok: bool) {
    if ok {
        ATTACHED.fetch_add(1, Ordering::Relaxed);
    } else {
        FAILED.fetch_add(1, Ordering::Relaxed);
        log::warn!(Audioware::env(), "Audioware hook {name}: Attach returned false");
    }
}

/// Counts a hook that is not attached on the Mac, with the reason (one log line).
pub fn skipped(name: &str, why: &str) {
    SKIPPED.fetch_add(1, Ordering::Relaxed);
    log::info!(Audioware::env(), "Audioware hook {name}: skipped on macOS ({why})");
}

/// One line at the end of `hooks::attach`.
pub fn summary(env: &SdkEnv) {
    log::info!(
        env,
        "Audioware hooks: mode {}, {} attached, {} failed, {} skipped",
        mode().as_str(),
        ATTACHED.load(Ordering::Relaxed),
        FAILED.load(Ordering::Relaxed),
        SKIPPED.load(Ordering::Relaxed)
    );
}

static STOP: Mutex<bool> = Mutex::new(false);
static WAKE: Condvar = Condvar::new();
static REPORTER: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

/// Starts the thread that logs the running counts of the probes every 60 seconds (probe mode).
pub fn start_reporter() {
    let mut slot = locked(&REPORTER);
    if slot.is_some() {
        return;
    }
    *locked(&STOP) = false;
    *slot = std::thread::Builder::new()
        .name("audioware-probe".into())
        .spawn(|| {
            let mut stop = locked(&STOP);
            loop {
                let (guard, timeout) = WAKE
                    .wait_timeout(stop, REPORT_EVERY)
                    .unwrap_or_else(PoisonError::into_inner);
                stop = guard;
                if *stop {
                    break;
                }
                if timeout.timed_out() {
                    report();
                }
            }
        })
        .ok();
}

/// Stops the reporter thread and waits for it (plugin unload).
pub fn stop_reporter() {
    let thread = locked(&REPORTER).take();
    if let Some(thread) = thread {
        *locked(&STOP) = true;
        WAKE.notify_all();
        let _ = thread.join();
    }
}

fn report() {
    let probes = locked(&PROBES);
    let mut total = 0;
    let mut called = Vec::new();
    for probe in probes.iter() {
        let n = probe.calls.load(Ordering::Relaxed);
        total += n;
        if n > 0 {
            called.push(format!("{}={n}", probe.name));
        }
    }
    log::info!(
        Audioware::env(),
        "probe report: {} of {} hooks called, {total} calls so far{}{}",
        called.len(),
        probes.len(),
        if called.is_empty() { "" } else { ": " },
        called.join(", ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mode_file_is_one_word() {
        assert_eq!(Mode::parse("off\n"), Some(Mode::Off));
        assert_eq!(Mode::parse("  Probe  "), Some(Mode::Probe));
        assert_eq!(Mode::parse("# comment\non\n"), Some(Mode::On));
        assert_eq!(Mode::parse(""), None);
        assert_eq!(Mode::parse("yes"), None);
    }

    #[test]
    fn raw_reads_at_most_eight_bytes() {
        assert_eq!(raw(&0x1122_3344_5566_7788u64), 0x1122_3344_5566_7788);
        assert_eq!(raw(&1u32), 1);
        assert_eq!(raw(&true), 1);
        assert_eq!(raw(&()), 0);
        assert_eq!(raw(&[1u8; 16]), 0x0101_0101_0101_0101);
    }
}
