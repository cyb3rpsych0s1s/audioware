use std::{ops::Range, path::PathBuf, time::Duration};

use audioware_bank::Banks;
use audioware_core::With;
use kira::{
    AudioManager, Decibels, Tween,
    backend::Backend,
    clock::{ClockHandle, ClockSpeed, ClockTime},
    sound::{
        FromFileError, PlaybackState,
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
    },
    track::{TrackBuilder, TrackHandle},
};

use crate::{engine::tweens::IMMEDIATELY, error::Error};

use super::ambience::Ambience;

pub struct Music {
    track: TrackHandle,
    main_menu_music: Option<Box<MainMenuMusic>>,
}

impl Music {
    pub fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        banks: &Banks,
        ambience: &Ambience,
    ) -> Result<Self, Error> {
        let mut track = manager.add_sub_track(
            TrackBuilder::new()
                // reverb used to require to be set otherwise sound switched to mono, what now?
                .with_send(ambience.reverb(), Decibels::SILENCE),
        )?;
        let main_menu_music = if let Some(main_menu) = banks.main_menu.as_ref() {
            let x = MainMenuMusic::try_new::<B>(manager, main_menu, &mut track)?;
            Some(Box::new(x))
        } else {
            None
        };
        Ok(Self {
            track,
            main_menu_music,
        })
    }
}

impl std::ops::Deref for Music {
    type Target = TrackHandle;

    fn deref(&self) -> &Self::Target {
        &self.track
    }
}

impl std::ops::DerefMut for Music {
    fn deref_mut(&mut self) -> &mut TrackHandle {
        &mut self.track
    }
}

impl AsRef<TrackHandle> for Music {
    fn as_ref(&self) -> &TrackHandle {
        &self.track
    }
}

impl<'a> From<&'a Music> for &'a TrackHandle {
    fn from(value: &'a Music) -> Self {
        &value.track
    }
}

#[allow(clippy::large_enum_variant, reason = "enum itself is boxed")]
pub enum MainMenuMusic {
    Sequence(Sequence),
    Crossfade(Crossfade),
}

impl MainMenuMusic {
    fn try_new<B: Backend>(
        manager: &mut AudioManager<B>,
        main_menu: &audioware_manifest::MainMenu,
        parent: &mut TrackHandle,
    ) -> Result<MainMenuMusic, Error> {
        let track = parent.add_sub_track(TrackBuilder::new())?;
        match main_menu.music.clone() {
            audioware_manifest::MainMenuMusic::SimpleLoop(file) => {
                Ok(MainMenuMusic::Sequence(Sequence {
                    file,
                    track,
                    settings: StreamingSoundSettings::default().loop_region(..),
                }))
            }
            audioware_manifest::MainMenuMusic::CustomLoop { file, settings } => {
                Ok(Self::Sequence(Sequence {
                    track,
                    file,
                    settings: StreamingSoundSettings::default().with(settings),
                }))
            }
            audioware_manifest::MainMenuMusic::Crossfade {
                file,
                volume,
                fade_in,
                fade_out,
                region,
            } => {
                let clock = manager.add_clock(ClockSpeed::TicksPerSecond(1.))?;
                let data = StreamingSoundData::from_file(&file)?;
                let duration = data.duration();
                let region = if let Some(region) = region {
                    region.starts.map(|x| x.as_secs_f64()).unwrap_or(0.)
                        ..region
                            .ends
                            .map(|x| x.as_secs_f64())
                            .unwrap_or(duration.as_secs_f64())
                } else {
                    0.0..duration.as_secs_f64()
                };
                Ok(Self::Crossfade(Crossfade {
                    current_fade_in: clock.time(),
                    current_fade_out: clock.time(),
                    next_fade_in: clock.time(),
                    next_fade_out: clock.time(),
                    track,
                    clock,
                    file,
                    region,
                    current: None,
                    next: None,
                    fade_in: fade_in.into(),
                    fade_out: fade_out.into(),
                    duration,
                    fade_in_at: Duration::default(),
                    fade_out_at: Duration::default(),
                    has_faded_once: false,
                }))
            }
        }
    }
    pub fn play<B: Backend>(&mut self, manager: &mut AudioManager<B>) -> Result<(), Error> {
        match self {
            Self::Sequence(sequence) => sequence.play(manager)?,
            Self::Crossfade(crossfade) => crossfade.play(manager)?,
        };
        Ok(())
    }
}

pub struct Sequence {
    track: TrackHandle,
    file: PathBuf,
    settings: StreamingSoundSettings,
}

impl Sequence {
    fn settings(&self) -> &StreamingSoundSettings {
        &self.settings
    }
    fn play<B: Backend>(&mut self, manager: &mut AudioManager<B>) -> Result<(), Error> {
        let data = StreamingSoundData::from_file(&self.file)?;
        manager.play(data)?;
        Ok(())
    }
}

pub struct Crossfade {
    track: TrackHandle,
    clock: ClockHandle,
    file: PathBuf,
    region: Range<f64>,
    current: Option<StreamingSoundHandle<FromFileError>>,
    next: Option<StreamingSoundHandle<FromFileError>>,
    fade_in: Tween,
    fade_out: Tween,
    duration: Duration,
    current_fade_in: ClockTime,
    current_fade_out: ClockTime,
    next_fade_in: ClockTime,
    next_fade_out: ClockTime,
    fade_in_at: Duration,
    fade_out_at: Duration,
    has_faded_once: bool,
}

impl Crossfade {
    pub fn play<B: Backend>(&mut self, manager: &mut AudioManager<B>) -> Result<(), Error> {
        let data = StreamingSoundData::from_file(&self.file)?;
        let duration = data.duration();
        self.current = Some(manager.play(data)?);
        self.clock.start();
        let started = self.clock.time();
        self.fade_in_at = duration - self.fade_in.duration;
        self.fade_out_at = duration - self.fade_out.duration;
        self.current_fade_in = started + self.fade_in_at_x2();
        self.current_fade_out = started + self.fade_out_at.as_secs_f64();
        self.next_fade_in = started + self.fade_in_at.as_secs_f64();
        self.next_fade_out = started + self.fade_out_at_x2();
        self.has_faded_once = false;
        self.duration = duration;
        Ok(())
    }
    pub fn update<B: Backend>(&mut self, manager: &mut AudioManager<B>) -> Result<(), Error> {
        if self.clock.time() >= self.current_fade_in {
            self.current_fade_out = self.current_fade_in + self.fade_out_at.as_secs_f64();
            self.next_fade_in = self.current_fade_in + self.fade_in_at.as_secs_f64();
            self.current_fade_in += self.fade_in_at_x2();
            let data = StreamingSoundData::from_file(&self.file)?;
            self.current = Some(manager.play(data.fade_in_tween(self.fade_in))?);
        }
        if self.clock.time() >= self.current_fade_out {
            self.current_fade_out += self.fade_out_at_x2();
            if let Some(x) = self.current.as_mut() {
                x.stop(self.fade_out)
            }
        }
        if self.clock.time() >= self.next_fade_in {
            self.next_fade_out = self.next_fade_in + self.fade_out_at.as_secs_f64();
            self.current_fade_in = self.next_fade_in + self.fade_in_at.as_secs_f64();
            self.next_fade_in += self.fade_in_at_x2();
            let data = StreamingSoundData::from_file(&self.file)?;
            self.next = Some(manager.play(data.fade_in_tween(self.fade_in))?);
            self.has_faded_once = true;
        }
        if self.clock.time() >= self.next_fade_out {
            self.next_fade_out += self.fade_out_at_x2();
            if let Some(x) = self.next.as_mut() {
                x.stop(self.fade_out)
            }
        }
        Ok(())
    }
    pub fn reset(&mut self) {
        self.clock.stop();
        if let Some(x) = self.current.as_mut() {
            x.stop(IMMEDIATELY)
        }
        if let Some(x) = self.next.as_mut() {
            x.stop(IMMEDIATELY)
        }
    }
    pub fn is_active(&self) -> bool {
        self.current
            .as_ref()
            .map(|x| x.state() != PlaybackState::Stopped)
            .unwrap_or(false)
            || self
                .next
                .as_ref()
                .map(|x| x.state() != PlaybackState::Stopped)
                .unwrap_or(false)
    }
    const fn fade_in_at_x2(&self) -> f64 {
        (self.duration.as_secs_f64() - self.fade_in.duration.as_secs_f64()) * 2.
    }
    const fn fade_out_at_x2(&self) -> f64 {
        (self.duration.as_secs_f64() - self.fade_out.duration.as_secs_f64()) * 2.
    }
}
