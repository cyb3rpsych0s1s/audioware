use audioware_sys::interop::locale::Locale;
use voice::Subtitle;

pub mod bank;
pub mod error;
pub mod id;
pub mod manifest;
pub mod redmod;
pub mod sfx;
pub mod voice;

pub trait Subtitles {
    fn subtitles(&self, locale: Locale) -> Vec<Subtitle<'_>>;
}
