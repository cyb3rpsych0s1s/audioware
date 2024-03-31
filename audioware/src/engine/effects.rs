use kira::{track::effect::filter::FilterHandle, CommandError};
use red4ext_rs::conv::NativeRepr;

/// suppress high frequences (from 20k to x)
pub const EQ_LOW_PASS_DEFAULT_FREQUENCES: f64 = 20_000.;
/// suppress low frequences (from 0 to x)
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
    fn preset(&mut self, preset: Preset) -> Result<(), CommandError>;
}

impl EqPass for LowPass {
    fn preset(&mut self, preset: Preset) -> Result<(), CommandError> {
        match preset {
            Preset::None => {
                self.0.set_mix(0., Default::default())?;
            }
            Preset::OnThePhone => {
                self.0
                    .set_cutoff(EQ_LOW_PASS_PHONE_CUTOFF, Default::default())?;
                self.0.set_resonance(EQ_RESONANCE, Default::default())?;
                self.0.set_mix(1., Default::default())?;
            }
            Preset::Underwater => {
                self.0
                    .set_cutoff(EQ_LOW_PASS_UNDERWATER_CUTOFF, Default::default())?;
                self.0.set_resonance(EQ_RESONANCE, Default::default())?;
                self.0.set_mix(1., Default::default())?;
            }
        }
        Ok(())
    }
}

impl EqPass for HighPass {
    fn preset(&mut self, preset: Preset) -> Result<(), CommandError> {
        match preset {
            Preset::None | Preset::Underwater => {
                self.0.set_mix(0., Default::default())?;
            }
            Preset::OnThePhone => {
                self.0
                    .set_cutoff(EQ_HIGH_PASS_PHONE_CUTOFF, Default::default())?;
                self.0.set_resonance(EQ_RESONANCE, Default::default())?;
                self.0.set_mix(1., Default::default())?;
            }
        }
        Ok(())
    }
}

impl EqPass for EQ {
    fn preset(&mut self, preset: Preset) -> Result<(), CommandError> {
        self.lowpass.preset(preset)?;
        self.highpass.preset(preset)?;
        red4ext_rs::info!("updated preset successfully to {preset}");
        Ok(())
    }
}
