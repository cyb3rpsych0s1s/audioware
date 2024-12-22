use audioware_manifest::Settings;
use kira::tween::Tween;
use red4ext_rs::types::CName;

use crate::engine::tweens::{DILATION_EASE_IN, DILATION_EASE_OUT, DILATION_LINEAR};

#[derive(Debug)]
pub struct Dilation {
    pub value: f32,
    pub last: Option<DilationUpdate>,
}

impl Dilation {
    pub fn new(value: f32) -> Self {
        Self { value, last: None }
    }
    pub fn dilation(&self) -> f64 {
        match self.last {
            Some(ref update) => update.dilation(),
            None => self.value as f64,
        }
    }
    pub fn tween(&self) -> Option<Tween> {
        self.last.as_ref().map(|x| x.tween_curve())
    }
}

#[derive(Debug, Clone)]
pub enum DilationUpdate {
    Set {
        value: f32,
        reason: CName,
        ease_in_curve: CName,
    },
    Unset {
        reason: CName,
        ease_out_curve: CName,
    },
}

impl DilationUpdate {
    pub fn dilation(&self) -> f64 {
        match self {
            Self::Set { value, .. } => *value as f64,
            Self::Unset { .. } => 1.,
        }
    }
    pub fn tween_curve(&self) -> Tween {
        if !self.has_curve() {
            DILATION_LINEAR
        } else {
            match self {
                Self::Set { .. } => DILATION_EASE_IN,
                Self::Unset { .. } => DILATION_EASE_OUT,
            }
        }
    }
}

impl DilationUpdate {
    pub fn has_curve(&self) -> bool {
        let curve = match self {
            Self::Set { ease_in_curve, .. } => ease_in_curve,
            Self::Unset { ease_out_curve, .. } => ease_out_curve,
        }
        .as_str();
        curve != "None" && !curve.is_empty()
    }
}

impl PartialEq for DilationUpdate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Set { value, reason, .. },
                Self::Set {
                    value: x,
                    reason: y,
                    ..
                },
            ) => *value == *x && *reason == *y,
            (Self::Unset { reason, .. }, Self::Unset { reason: y, .. }) => *reason == *y,
            _ => false,
        }
    }
}

pub trait AffectedByTimeDilation {
    fn affected_by_time_dilation(&self) -> bool;
}

impl AffectedByTimeDilation for Settings {
    #[inline(always)]
    fn affected_by_time_dilation(&self) -> bool {
        self.affected_by_time_dilation.unwrap_or(true)
    }
}

impl AffectedByTimeDilation for Tween {
    #[inline(always)]
    fn affected_by_time_dilation(&self) -> bool {
        true
    }
}
