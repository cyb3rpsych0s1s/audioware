use red4ext_rs::{class_kind::Scripted, ScriptClass, ScriptClassOps};

/// engine audio backend buffer size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(i64)]
#[allow(dead_code)]
pub enum AudiowareBufferSize {
    #[default]
    Auto = 0,
    Option64 = 64,
    Option128 = 128,
    Option256 = 256,
    Option512 = 512,
    Option1024 = 1024,
}

pub struct AudiowareConfig {
    pub buffer_size: AudiowareBufferSize,
}
unsafe impl ScriptClass for AudiowareConfig {
    type Kind = Scripted;
    const NAME: &'static str = "AudiowareConfig";
}

pub fn buffer_size() -> Option<AudiowareBufferSize> {
    if let Some(config) = AudiowareConfig::new_ref() {
        return unsafe { config.fields() }.map(|x| x.buffer_size);
    }
    None
}
