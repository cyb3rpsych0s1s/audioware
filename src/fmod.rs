use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use libfmod::{
    ffi::{
        FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL, FMOD_STUDIO_SYSTEM,
    },
    SpeakerMode, Studio,
};
use red4ext_rs::ffi::CName;
use red4ext_rs::prelude::*;
use static_init::dynamic;

#[dynamic]
static mut MODS: HashMap<CName, Vec<String>> = HashMap::new();

#[dynamic]
static mut HANDLE: Option<&'static mut FMOD_STUDIO_SYSTEM> = None;

macro_rules! on_error {
    ($e:expr) => {
        let message = match $e {
            libfmod::Error::Fmod { .. } => "internal error".to_string(),
            libfmod::Error::EnumBindgen { .. } => "bindgen error".to_string(),
            libfmod::Error::String(e) => e.into_cstring().to_str().unwrap_or("invalid string").to_string(),
            libfmod::Error::StringNul(_) => "null string error".to_string(),
            libfmod::Error::NotDspFft => "not dsp fft error".to_string(),
        };
        call!("Audioware.Utils::F;String" (message) -> ());
    };
}

macro_rules! report {
    ($m:literal) => {
        call!("Audioware.Utils::E;String" ($m) -> ());
    };
}

pub(crate) fn load(name: String) -> Option<Studio> {
    let studio = get_studio();
    if let Err(e) = studio {
        on_error!(e);
        return None;
    }
    // SAFETY: error case tested above
    let studio = studio.unwrap();
    if let Some(folder) = get_mod_custom_sounds_path(name.as_str()) {
        studio
            .load_bank_file(
                folder
                    .join(name.as_str())
                    .with_extension("bank")
                    .to_str()
                    .unwrap(),
                FMOD_STUDIO_LOAD_BANK_NORMAL,
            )
            .expect("error loading bank");
        studio
            .load_bank_file(
                folder
                    .join(name.as_str())
                    .with_extension("strings.bank")
                    .to_str()
                    .unwrap(),
                FMOD_STUDIO_LOAD_BANK_NORMAL,
            )
            .expect("error loading bank strings");
        return Some(studio);
    }
    None
}

pub(crate) fn unload() {}

pub(crate) fn get_studio() -> Result<Studio, libfmod::Error> {
    if let Some(handle) = &mut *HANDLE.write() {
        let ptr: &mut FMOD_STUDIO_SYSTEM = handle;
        let ptr = ptr as *mut FMOD_STUDIO_SYSTEM;
        let studio = Studio::from(ptr);
        report!("get_studio from HANDLE");
        return Ok(studio);
    }
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    // SAFETY: it has just been created
    unsafe {
        *HANDLE.write() = Some(&mut *studio.as_mut_ptr());
    }
    std::mem::forget(studio);
    report!("get_studio new HANDLE");
    Ok(studio)
}

fn get_mod_custom_sounds_path(folder: &str) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let folder: &Path = folder.as_ref();
    let path = exe
        .parent()?
        .parent()?
        .parent()?
        .join("mods")
        .join(folder)
        .join("customSounds");
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::get_studio;

    #[test]
    fn singleton() {
        let studio = get_studio().expect("get studio");
        std::mem::forget(studio);
        let another = get_studio().expect("get studio");
        assert_eq!(studio, another);
        std::mem::drop(studio);
    }
}
