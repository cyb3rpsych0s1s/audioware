use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use libfmod::{
    ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_SYSTEM},
    SpeakerMode, Studio,
};
use red4ext_rs::ffi::CName;
use red4ext_rs::prelude::*;
use static_init::dynamic;

#[dynamic]
static mut MODS: HashMap<CName, Vec<String>> = HashMap::new();

#[dynamic]
static mut HANDLE: Arc<Mutex<Option<&'static mut FMOD_STUDIO_SYSTEM>>> = Arc::new(Mutex::new(None));

macro_rules! on_error {
    ($e:expr) => {
        let message = match $e {
            ::libfmod::Error::Fmod { function, code, message } => format!("internal error\nfunction {function:#?}\ncode {code:#?}\nmessage {message}"),
            ::libfmod::Error::EnumBindgen { .. } => "bindgen error".to_string(),
            ::libfmod::Error::String(e) => e.into_cstring().to_str().unwrap_or("invalid string").to_string(),
            ::libfmod::Error::StringNul(_) => "null string error".to_string(),
            ::libfmod::Error::NotDspFft => "not dsp fft error".to_string(),
        };
        if cfg!(test) {
            println!("calling Audioware.Utils::F;String (\"{}\") -> ()", message);
        } else {
            call!("Audioware.Utils::F;String" (message) -> ());
        }
    };
}

macro_rules! report {
    ($m:literal) => {
        if cfg!(test) {
            println!("calling Audioware.Utils::E;String (\"{}\") -> ()", $m);
        } else {
            call!("Audioware.Utils::E;String" ($m) -> ());
        }
    };
}

pub(crate) fn load(name: String) -> anyhow::Result<Arc<Mutex<Option<&'static mut FMOD_STUDIO_SYSTEM>>>> {
    let studio = get_studio();
    println!("studio {studio:#?}");
    if let Err(e) = studio {
        on_error!(e);
        return Err(anyhow!("error while getting handle"));
    }
    // SAFETY: error case tested above
    let studio = studio.unwrap();
    if let Some(folder) = get_mod_custom_sounds_path(name.as_str()) {
        println!("folder {folder:#?}");
        let files = std::fs::read_dir(folder.join("customSounds"));
        if let Err(e) = files {
            println!("error {e:#?}");
            return Err(anyhow!("error while reading dir"));
        }
        let files = files.unwrap();
        let mut wavs: Vec<PathBuf> = vec![];
        let mut banks: Vec<PathBuf> = vec![];
        files.into_iter().for_each(|x| {
            if let Ok(ref file) = x {
                if let Some(filename) = file.file_name().to_str() {
                    if let Ok(ref metadata) = file.metadata() {
                        if metadata.is_file() && !metadata.is_symlink() && filename.ends_with(".wav") {
                            wavs.push(filename.try_into().expect("filename as path buf"));
                            return;
                        }
                        if metadata.is_file() && !metadata.is_symlink() && filename.ends_with(".bank") {
                            banks.push(filename.try_into().expect("filename as path buf"));
                            return;
                        }
                    }
                }
            }
        });
        println!(".wav {wavs:#?}");
        println!(".bank {banks:#?}");
        // studio
        //     .load_bank_file(
        //         folder
        //             .join(name.as_str())
        //             .with_extension("bank")
        //             .to_str()
        //             .unwrap(),
        //         FMOD_STUDIO_LOAD_BANK_NORMAL,
        //     )
        //     .expect("error loading bank");
        // studio
        //     .load_bank_file(
        //         folder
        //             .join(name.as_str())
        //             .with_extension("strings.bank")
        //             .to_str()
        //             .unwrap(),
        //         FMOD_STUDIO_LOAD_BANK_NORMAL,
        //     )
        //     .expect("error loading bank strings");
        return Ok(studio);
    }
    Err(anyhow!("couldn't get folder path"))
}

pub(crate) fn unload() {}

pub(crate) fn get_studio() -> Result<Arc<Mutex<Option<&'static mut FMOD_STUDIO_SYSTEM>>>, libfmod::Error> {
    let binding = (*HANDLE.write()).clone();
    let inner = &mut *binding.lock().expect("acquire mutex");
    if let Some(handle) = inner {
        let ptr: &mut FMOD_STUDIO_SYSTEM = handle;
        let ptr = ptr as *mut FMOD_STUDIO_SYSTEM;
        report!("before it turn the HANDLE into a Studio");
        let studio = Studio::from(ptr);
        report!("after it turned the HANDLE into a Studio");
        if studio.is_valid() {
            report!(">> return valid HANDLE");
            return Ok(binding.clone());
        }
    }
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    // SAFETY: it has just been created and it's behind a mutex guard
    unsafe {
        *inner = Some(&mut *studio.as_mut_ptr());
    }
    std::mem::forget(studio);
    report!(">> return new HANDLE");
    Ok(binding.clone())
}

#[cfg(not(test))]
fn get_mod_custom_sounds_path(folder: &str) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let folder: &Path = folder.as_ref();
    let path = exe.parent()?.parent()?.parent()?.join("mods").join(folder);
    Some(path)
}

#[cfg(test)]
fn get_mod_custom_sounds_path(folder: &str) -> Option<PathBuf> {
    let exe = PathBuf::from(
        std::env::var("LOCAL_REPO")
            .expect("please define a LOCAL_REPO env var which points to the root of this repo"),
    );
    let folder: &Path = folder.as_ref();
    let path = exe.join("mock").join("mods").join(folder);
    Some(path)
}

#[cfg(test)]
mod tests {
    use libfmod::{ffi::FMOD_STUDIO_SYSTEM, Studio};
    use serial_test::serial;

    use super::{get_studio, load};

    #[test]
    #[serial]
    fn singleton() {
        println!("before studio in singleton");
        let studio = get_studio().expect("get studio");
        println!("before another in singleton");
        let another = get_studio().expect("get studio");
        let studio = *studio.clone().lock().unwrap().as_mut().unwrap() as *mut FMOD_STUDIO_SYSTEM;
        let another = *another.clone().lock().unwrap().as_mut().unwrap() as *mut FMOD_STUDIO_SYSTEM;
        assert_eq!(studio, another);
        let studio = Studio::from(studio);
        let another = Studio::from(another);
        let success = studio.release();
        let failure = another.release();
        assert!(success.is_ok());
        assert!(failure.is_err());
    }

    #[test]
    #[serial]
    fn initialize() {
        println!("before another in initialize");
        let studio = load("fakemod".to_string());
        assert!(studio.is_ok());
        let studio = *studio.unwrap().clone().lock().unwrap().as_mut().unwrap() as *mut FMOD_STUDIO_SYSTEM;
        assert_ne!(studio, std::ptr::null_mut());
        let studio = Studio::from(studio);
        let _ = studio.release();
    }
}
