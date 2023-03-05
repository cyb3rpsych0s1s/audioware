use libfmod::{
    ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL},
    Error, SpeakerMode, Studio,
};

#[dynamic]
static mut HANDLE: Option<Studio> = None;

pub(crate) fn load() -> &Studio {
    unsafe {
        if let None = HANDLE {
            let studio = Studio::create()?;
            let system = studio.get_core_system()?;
            system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
            studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
            HANDLE = Some(studio);
        }
        return &HANDLE;
    }
}

pub(crate) fn unload() {
    unsafe {
        if let Some(ref handle) = HANDLE {
            handle.release()?;
        }
        HANDLE = None;
    }
}
