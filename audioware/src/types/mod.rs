use red4ext_rs::types::CName;

pub mod bank;
pub mod id;
pub mod redmod;
pub mod sfx;
pub mod voice;

pub trait GetRaw {
    type Output;
    fn get_raw(&self, raw: &CName) -> Option<&Self::Output>;
}
