use kira::effect::filter::FilterHandle;
use red4ext_rs::NativeRepr;

use crate::utils;

use super::tweens::DEFAULT;

/// suppress high frequences (from 20k to x)
#[allow(dead_code)]
pub const EQ_LOW_PASS_DEFAULT_FREQUENCES: f64 = 20_000.;
/// suppress low frequences (from 0 to x)
#[allow(dead_code)]
pub const EQ_HIGH_PASS_DEFAULT_FREQUENCES: f64 = 0.;

pub const EQ_LOW_PASS_PHONE_CUTOFF: f64 = 5_000.;
pub const EQ_HIGH_PASS_PHONE_CUTOFF: f64 = 500.;
pub const EQ_RESONANCE: f64 = 6.;

pub const EQ_LOW_PASS_UNDERWATER_CUTOFF: f64 = 500.;

pub struct EQ {
    pub lowpass: LowPass,
    pub highpass: HighPass,
}

pub struct LowPass(pub FilterHandle);
pub struct HighPass(pub FilterHandle);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(i64)]
pub enum Preset {
    #[default]
    None = 0,
    Underwater = 1,
    OnThePhone = 2,
}

impl std::fmt::Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "no preset"),
            Self::Underwater => write!(f, "underwater preset"),
            Self::OnThePhone => write!(f, "holocall preset"),
        }
    }
}

unsafe impl NativeRepr for Preset {
    const NAME: &'static str = "Audioware.Preset";
}

pub trait EqPass {
    fn set_preset(&mut self, preset: Preset);
}

impl EqPass for LowPass {
    fn set_preset(&mut self, preset: Preset) {
        match preset {
            Preset::None => {
                self.0.set_mix(0., DEFAULT);
            }
            Preset::OnThePhone => {
                self.0.set_cutoff(EQ_LOW_PASS_PHONE_CUTOFF, DEFAULT);
                self.0.set_resonance(EQ_RESONANCE, DEFAULT);
                self.0.set_mix(1., DEFAULT);
            }
            Preset::Underwater => {
                self.0.set_cutoff(EQ_LOW_PASS_UNDERWATER_CUTOFF, DEFAULT);
                self.0.set_resonance(EQ_RESONANCE, DEFAULT);
                self.0.set_mix(1., DEFAULT);
            }
        }
    }
}

impl EqPass for HighPass {
    fn set_preset(&mut self, preset: Preset) {
        match preset {
            Preset::None | Preset::Underwater => {
                self.0.set_mix(0., DEFAULT);
            }
            Preset::OnThePhone => {
                self.0.set_cutoff(EQ_HIGH_PASS_PHONE_CUTOFF, DEFAULT);
                self.0.set_resonance(EQ_RESONANCE, DEFAULT);
                self.0.set_mix(1., DEFAULT);
            }
        }
    }
}

impl EqPass for EQ {
    fn set_preset(&mut self, preset: Preset) {
        self.lowpass.set_preset(preset);
        self.highpass.set_preset(preset);
        utils::silly!("updated preset successfully to {preset}");
    }
}
