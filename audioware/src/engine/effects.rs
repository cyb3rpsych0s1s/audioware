use kira::{track::effect::eq_filter::EqFilterHandle, CommandError};
use red4ext_rs::conv::NativeRepr;

/// suppress high frequences (from 20k to x)
pub const EQ_LOW_PASS_DEFAULT_FREQUENCES: f64 = 20_000.;
/// suppress low frequences (from 0 to x)
pub const EQ_HIGH_PASS_DEFAULT_FREQUENCES: f64 = 0.;
pub const EQ_DEFAULT_GAIN: f64 = 0.;
pub const EQ_DEFAULT_Q: f64 = 0.;

pub struct EQ {
    pub lowpass: LowPass,
    pub highpass: HighPass,
}

pub struct LowPass(pub EqFilterHandle);
pub struct HighPass(pub EqFilterHandle);

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
                self.0
                    .set_frequency(EQ_LOW_PASS_DEFAULT_FREQUENCES, Default::default())?;
                self.0.set_gain(EQ_DEFAULT_GAIN, Default::default())?;
                self.0.set_q(EQ_DEFAULT_Q, Default::default())?;
            }
            Preset::OnThePhone => {
                self.0.set_frequency(5000., Default::default())?;
                self.0.set_gain(6., Default::default())?;
                self.0.set_q(2., Default::default())?;
            }
            Preset::Underwater => {
                self.0.set_frequency(1_000., Default::default())?;
                self.0.set_gain(0., Default::default())?;
                self.0.set_q(0., Default::default())?;
            }
        }
        Ok(())
    }
}

impl EqPass for HighPass {
    fn preset(&mut self, preset: Preset) -> Result<(), CommandError> {
        match preset {
            Preset::None | Preset::Underwater => {
                self.0
                    .set_frequency(EQ_HIGH_PASS_DEFAULT_FREQUENCES, Default::default())?;
                self.0.set_gain(EQ_DEFAULT_GAIN, Default::default())?;
                self.0.set_q(EQ_DEFAULT_Q, Default::default())?;
            }
            Preset::OnThePhone => {
                self.0.set_frequency(500., Default::default())?;
                self.0.set_gain(6., Default::default())?;
                self.0.set_q(2., Default::default())?;
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
