use kira::{track::effect::eq_filter::EqFilterHandle, CommandError};
use red4ext_rs::conv::NativeRepr;

/// suppress high frequences (from 20k to x)
pub const EQ_LOW_PASS_DEFAULT_FREQUENCES: f64 = 20_000.;
/// suppress low frequences (from 0 to x)
pub const EQ_HIGH_PASS_DEFAULT_FREQUENCES: f64 = 0.;
pub const EQ_DEFAULT_GAIN: f64 = 0.;
pub const EQ_DEFAULT_Q: f64 = 0.;

pub const EQ_LOW_PASS_PHONE_FREQUENCES: f64 = 5000.;
pub const EQ_HIGH_PASS_PHONE_FREQUENCES: f64 = 500.;
pub const EQ_PHONE_GAIN: f64 = 6.;
pub const EQ_PHONE_Q: f64 = 2.;

pub const EQ_LOW_PASS_UNDERWATER_FREQUENCES: f64 = 1_000.;
pub const EQ_UNDERWATER_GAIN: f64 = EQ_DEFAULT_GAIN;
pub const EQ_UNDERWATER_Q: f64 = EQ_DEFAULT_Q;

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
                self.0
                    .set_frequency(EQ_LOW_PASS_PHONE_FREQUENCES, Default::default())?;
                self.0.set_gain(EQ_PHONE_GAIN, Default::default())?;
                self.0.set_q(EQ_PHONE_Q, Default::default())?;
            }
            Preset::Underwater => {
                self.0
                    .set_frequency(EQ_LOW_PASS_UNDERWATER_FREQUENCES, Default::default())?;
                self.0.set_gain(EQ_UNDERWATER_GAIN, Default::default())?;
                self.0.set_q(EQ_UNDERWATER_Q, Default::default())?;
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
                self.0
                    .set_frequency(EQ_HIGH_PASS_PHONE_FREQUENCES, Default::default())?;
                self.0.set_gain(EQ_PHONE_GAIN, Default::default())?;
                self.0.set_q(EQ_PHONE_Q, Default::default())?;
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
