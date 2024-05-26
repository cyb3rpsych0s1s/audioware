use red4ext_rs::types::CName;

pub mod bank;
pub mod error;
pub mod id;
pub mod redmod;
pub mod sfx;
pub mod voice;

pub trait GetByCName {
    type Output;
    fn get_by_cname(&self, raw: &CName) -> Option<&Self::Output>;
}
