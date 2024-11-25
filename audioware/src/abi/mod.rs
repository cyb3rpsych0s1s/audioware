use std::time::Duration;

use audioware_manifest::{
    Interpolation, Locale, LocaleExt, PlayerGender, Region, ScnDialogLineType, Settings,
};
use command::Command;
use crossbeam::channel::bounded;
use kira::{manager::backend::cpal::CpalBackend, tween::Easing};
use lifecycle::{Board, Codeware, Lifecycle, Session, System};
use red4ext_rs::{
    class_kind::{Native, Scripted},
    exports, methods,
    types::{CName, EntityId, IScriptable, Opt, Ref, StaticArray},
    ClassExport, Exportable, GameApp, RttiRegistrator, ScriptClass, SdkEnv, StateListener,
    StateType,
};

use crate::{
    engine::{eq::Preset, Engine},
    queue,
    utils::{fails, lifecycle, warns},
    Audioware, ElasticTween, EmitterDistances, EmitterSettings, LinearTween, LocalizationPackage,
    ToEasing, ToTween, Tween, AUDIOWARE_VERSION,
};

pub mod command;
pub mod lifecycle;

/// Register [plugin][Plugin] lifecycle listeners.
pub fn register_listeners(env: &SdkEnv) {
    RttiRegistrator::add(Some(register), Some(post_register));
    env.add_listener(
        StateType::Initialization,
        StateListener::default().with_on_exit(on_exit_initialization),
    );
    env.add_listener(
        StateType::Running,
        StateListener::default().with_on_exit(on_exit_running),
    );
}

macro_rules! g {
    ($reds:literal, $rust:path) => {
        ::red4ext_rs::GlobalExport(::red4ext_rs::global!($reds, $rust))
    };
}

/// Register types in [RTTI][RttiSystem].
#[allow(clippy::transmute_ptr_to_ref)] // upstream lint
#[rustfmt::skip]
pub fn exports() -> impl Exportable {
    let exp = exports![
        ClassExport::<AudioSystemExt>::builder()
                .base(IScriptable::NAME)
                .methods(methods![
                    final c"Play" => AudioSystemExt::play,
                    final c"Stop" => AudioSystemExt::stop,
                    final c"Switch" => AudioSystemExt::switch,
                    final c"PlayOverThePhone" => AudioSystemExt::play_over_the_phone,
                    final c"PlayOnEmitter" => AudioSystemExt::play_on_emitter,
                    final c"StopOnEmitter" => AudioSystemExt::stop_on_emitter,
                    final c"RegisterEmitter" => AudioSystemExt::register_emitter,
                    final c"UnregisterEmitter" => AudioSystemExt::unregister_emitter,
                    final c"IsRegisteredEmitter" => AudioSystemExt::is_registered_emitter,
                    final c"OnEmitterDies" => AudioSystemExt::on_emitter_dies,
                    final c"OnEmitterIncapacitated" => AudioSystemExt::on_emitter_incapacitated,
                    final c"OnEmitterDefeated" => AudioSystemExt::on_emitter_defeated,
                    final c"EmittersCount" => AudioSystemExt::emitters_count,
                    final c"Duration" => AudioSystemExt::duration,
                    final c"IsDebug" => AudioSystemExt::is_debug,
                    final c"SemanticVersion" => AudioSystemExt::semantic_version,
                ])
                .build(),
        ClassExport::<DummyLol>::builder().base(IScriptable::NAME)
        .methods(methods![
            final c"Hi" => DummyLol::hi,
        ])
        .build(),
        g!(c"Audioware.OnGameSessionBeforeStart",   Audioware::on_game_session_before_start),
        g!(c"Audioware.OnGameSessionStart",         Audioware::on_game_session_start),
        g!(c"Audioware.OnGameSessionReady",         Audioware::on_game_session_ready),
        g!(c"Audioware.OnGameSessionPause",         Audioware::on_game_session_pause),
        g!(c"Audioware.OnGameSessionResume",        Audioware::on_game_session_resume),
        g!(c"Audioware.OnGameSessionBeforeEnd",     Audioware::on_game_session_before_end),
        g!(c"Audioware.OnGameSessionEnd",           Audioware::on_game_session_end),
        g!(c"Audioware.OnGameSystemAttach",         Audioware::on_game_system_attach),
        g!(c"Audioware.OnGameSystemDetach",         Audioware::on_game_system_detach),
        g!(c"Audioware.OnGameSystemPlayerAttach",   Audioware::on_game_system_player_attach),
        g!(c"Audioware.OnGameSystemPlayerDetach",   Audioware::on_game_system_player_detach),
        g!(c"Audioware.OnUIMenu",                   Audioware::on_ui_menu),
        g!(c"Audioware.SetReverbMix",               Audioware::on_reverb_mix),
        g!(c"Audioware.SetPreset",                  Audioware::on_preset),
        g!(c"Audioware.SetVolume",                  Audioware::set_volume),
        g!(c"Audioware.SetPlayerGender",            Audioware::set_player_gender),
        g!(c"Audioware.UnsetPlayerGender",          Audioware::unset_player_gender),
        g!(c"Audioware.SetGameLocales",             Audioware::set_game_locales),
        g!(c"Audioware.DefineSubtitles",            Audioware::define_subtitles),
        g!(c"Audioware.SupportedLanguages",         Audioware::supported_languages),
    ];
    #[cfg(not(feature = "hot-reload"))]
    {exp}
    #[cfg(feature = "hot-reload")]
    {
        use crate::abi::debug::HotReload;
        exports![exp, g!(c"HotReload", Audioware::hot_reload)]
    }
}

/// On RTTI registration.
unsafe extern "C" fn register() {
    lifecycle!("on RTTI register");
}

/// Once RTTI registered.
unsafe extern "C" fn post_register() {
    lifecycle!("on RTTI post register");
    Audioware::once_rtti_registered();
}

/// Once plugin initialized.
unsafe extern "C" fn on_exit_initialization(_: &GameApp) {
    lifecycle!("on plugin exit initialization");
    Audioware::once_exit_initialization();
}

/// Unload [Plugin].
unsafe extern "C" fn on_exit_running(_: &GameApp) {
    queue::notify(Lifecycle::Terminate);
}

pub trait GameSessionLifecycle {
    fn on_game_session_before_start();
    fn on_game_session_start();
    fn on_game_session_ready();
    fn on_game_session_pause();
    fn on_game_session_resume();
    fn on_game_session_before_end();
    fn on_game_session_end();
}

pub trait GameSystemLifecycle {
    fn on_game_system_attach();
    fn on_game_system_detach();
    fn on_game_system_player_attach();
    fn on_game_system_player_detach();
}

pub trait BlackboardLifecycle {
    fn on_ui_menu(value: bool);
    fn on_reverb_mix(value: f32);
    fn on_preset(value: Preset);
}

pub trait CodewareLifecycle {
    fn set_player_gender(gender: PlayerGender);
    fn unset_player_gender();
    fn set_game_locales(spoken: CName, written: CName);
    fn define_subtitles(package: Ref<LocalizationPackage>);
    fn supported_languages() -> Vec<CName>;
}

pub trait ListenerLifecycle {
    fn set_volume(setting: CName, value: f64);
}

impl GameSessionLifecycle for Audioware {
    fn on_game_session_before_start() {
        queue::notify(Lifecycle::Session(Session::BeforeStart));
    }

    fn on_game_session_start() {
        queue::notify(Lifecycle::Session(Session::Start));
    }

    fn on_game_session_ready() {
        queue::notify(Lifecycle::Session(Session::Ready));
    }

    fn on_game_session_pause() {
        queue::notify(Lifecycle::Session(Session::Pause));
    }

    fn on_game_session_resume() {
        queue::notify(Lifecycle::Session(Session::Resume));
    }

    fn on_game_session_before_end() {
        queue::notify(Lifecycle::Session(Session::BeforeEnd));
    }

    fn on_game_session_end() {
        queue::notify(Lifecycle::Session(Session::End));
    }
}

impl GameSystemLifecycle for Audioware {
    fn on_game_system_attach() {
        queue::notify(Lifecycle::System(System::Attach));
    }

    fn on_game_system_detach() {
        queue::notify(Lifecycle::System(System::Detach));
    }

    fn on_game_system_player_attach() {
        queue::notify(Lifecycle::System(System::PlayerAttach));
    }

    fn on_game_system_player_detach() {
        queue::notify(Lifecycle::System(System::PlayerDetach));
    }
}

impl BlackboardLifecycle for Audioware {
    fn on_ui_menu(value: bool) {
        queue::notify(Lifecycle::Board(Board::UIMenu(value)));
    }

    fn on_reverb_mix(value: f32) {
        queue::notify(Lifecycle::Board(Board::ReverbMix(value)));
    }

    fn on_preset(value: Preset) {
        queue::notify(Lifecycle::Board(Board::Preset(value)));
    }
}

impl ListenerLifecycle for Audioware {
    fn set_volume(setting: CName, value: f64) {
        queue::notify(Lifecycle::SetVolume { setting, value });
    }
}

impl CodewareLifecycle for Audioware {
    fn set_player_gender(gender: PlayerGender) {
        queue::notify(Lifecycle::Codeware(Codeware::SetPlayerGender { gender }));
    }

    fn unset_player_gender() {
        queue::notify(Lifecycle::Codeware(Codeware::UnsetPlayerGender));
    }

    fn set_game_locales(spoken: CName, written: CName) {
        queue::notify(Lifecycle::Codeware(Codeware::SetGameLocales {
            spoken,
            written,
        }));
    }

    fn define_subtitles(package: Ref<LocalizationPackage>) {
        Engine::<CpalBackend>::define_subtitles(package);
    }

    fn supported_languages() -> Vec<CName> {
        Engine::<CpalBackend>::supported_languages()
    }
}

pub trait SceneLifecycle {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Ref<EmitterSettings>,
    ) -> bool;
    fn unregister_emitter(&self, entity_id: EntityId) -> bool;
    fn is_registered_emitter(&self, entity_id: EntityId) -> bool;
    fn on_emitter_dies(&self, entity_id: EntityId);
    fn on_emitter_incapacitated(&self, entity_id: EntityId);
    fn on_emitter_defeated(&self, entity_id: EntityId);
    fn emitters_count(&self) -> i32;
}

impl SceneLifecycle for AudioSystemExt {
    fn register_emitter(
        &self,
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Ref<EmitterSettings>,
    ) -> bool {
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::RegisterEmitter {
            entity_id,
            emitter_name: emitter_name.into_option(),
            emitter_settings: emitter_settings.into_settings(),
            sender,
        });
        if let Ok(registered) = receiver.recv() {
            return registered;
        }
        false
    }

    fn unregister_emitter(&self, entity_id: EntityId) -> bool {
        let (sender, receiver) = bounded(0);
        queue::notify(Lifecycle::UnregisterEmitter { entity_id, sender });
        if let Ok(unregistered) = receiver.recv() {
            return unregistered;
        }
        false
    }

    fn is_registered_emitter(&self, entity_id: EntityId) -> bool {
        Engine::<CpalBackend>::is_registered_emitter(entity_id)
    }

    fn on_emitter_dies(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterDies { entity_id });
    }

    fn on_emitter_incapacitated(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterIncapacitated { entity_id });
    }

    fn on_emitter_defeated(&self, entity_id: EntityId) {
        queue::notify(Lifecycle::OnEmitterDefeated { entity_id });
    }

    fn emitters_count(&self) -> i32 {
        Engine::<CpalBackend>::emitters_count()
    }
}

/// Represents a region in time.
/// Useful to describe a portion of a sound.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioRegion {
    starts: f32,
    ends: f32,
}

unsafe impl ScriptClass for AudioRegion {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudioRegion";
}

/// Extended audio settings.
#[derive(Clone)]
#[repr(C)]
pub struct AudioSettingsExt {
    start_position: f32,
    region: Ref<AudioRegion>,
    r#loop: bool,
    volume: f32,
    fade_in: Ref<Tween>,
    panning: f32,
    playback_rate: f32,
    affected_by_time_dilation: bool,
}

impl Default for AudioSettingsExt {
    fn default() -> Self {
        Self {
            start_position: 0.,
            region: Ref::default(),
            r#loop: false,
            volume: 100.,
            fade_in: Ref::default(),
            panning: 0.5,
            playback_rate: 1.,
            affected_by_time_dilation: true,
        }
    }
}

unsafe impl ScriptClass for AudioSettingsExt {
    type Kind = Scripted;
    const NAME: &'static str = "Audioware.AudioSettingsExt";
}

impl std::fmt::Debug for AudioSettingsExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioSettingsExt")
            .field("start_position", &self.start_position)
            .field("loop", &self.r#loop)
            .field("volume", &self.volume)
            .field("panning", &self.panning)
            .field("playback_rate", &self.playback_rate)
            .finish_non_exhaustive()
    }
}

pub trait ToSettings {
    type Settings;
    fn into_settings(self) -> Option<Self::Settings>;
}

pub trait ToRegion {
    fn into_region(self) -> Option<Region>;
}

pub trait ToInterpolation {
    fn into_interpolation(self) -> Option<Interpolation>;
}

impl ToInterpolation for Ref<Tween> {
    fn into_interpolation(self) -> Option<Interpolation> {
        if self.is_null() {
            return None;
        }
        match self.clone() {
            x if x.is_a::<LinearTween>() => {
                let x = x.cast::<LinearTween>().unwrap();
                let x = unsafe { x.fields() }?;
                Some(Interpolation {
                    start_time: x
                        .start_time()
                        .ne(&0.)
                        .then_some(Duration::from_secs_f32(x.start_time())),
                    duration: Duration::from_secs_f32(x.duration()),
                    easing: Easing::Linear,
                })
            }
            x if x.is_a::<ElasticTween>() => {
                let x = x.cast::<ElasticTween>().unwrap();
                let x = unsafe { x.fields() }?;
                Some(Interpolation {
                    start_time: x
                        .start_time()
                        .ne(&0.)
                        .then_some(Duration::from_secs_f32(x.start_time())),
                    duration: Duration::from_secs_f32(x.duration()),
                    easing: match x.easing {
                        crate::Easing::InPowf => Easing::InPowf(x.value as f64),
                        crate::Easing::OutPowf => Easing::OutPowf(x.value as f64),
                        crate::Easing::InOutPowf => Easing::InOutPowf(x.value as f64),
                    },
                })
            }
            _ => unreachable!(),
        }
    }
}

impl ToRegion for Ref<AudioRegion> {
    fn into_region(self) -> Option<Region> {
        if self.is_null() {
            return None;
        }
        let AudioRegion { starts, ends } = unsafe { self.fields() }?.clone();
        if starts == 0. && ends == 0. {
            return None;
        }
        Some(Region {
            starts: starts.ne(&0.).then_some(Duration::from_secs_f32(starts)),
            ends: ends.ne(&0.).then_some(Duration::from_secs_f32(ends)),
        })
    }
}

impl ToSettings for Ref<AudioSettingsExt> {
    type Settings = Settings;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let AudioSettingsExt {
            start_position,
            region,
            r#loop,
            volume,
            fade_in,
            panning,
            playback_rate,
            affected_by_time_dilation,
        } = unsafe { self.fields() }?.clone();
        let mut settings = Settings::default();
        if start_position != 0.0 {
            settings.start_position = Some(Duration::from_secs_f32(start_position));
        }
        settings.region = region.into_region();
        if r#loop {
            settings.r#loop = Some(true);
        }
        if volume != 100. {
            settings.volume = Some(volume as f64);
        }
        settings.fade_in_tween = fade_in.into_interpolation();
        if panning != 0.5 {
            settings.panning = Some(panning as f64);
        }
        if playback_rate != 1.0 {
            settings.playback_rate = Some(kira::sound::PlaybackRate::Factor(playback_rate as f64));
        }
        if !affected_by_time_dilation {
            settings.affected_by_time_dilation = Some(false);
        }
        Some(settings)
    }
}

impl ToSettings for Ref<EmitterSettings> {
    type Settings = kira::spatial::emitter::EmitterSettings;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let EmitterSettings {
            distances,
            attenuation_function,
            enable_spatialization,
            persist_until_sounds_finish,
        } = unsafe { self.fields() }?.clone();
        Some(kira::spatial::emitter::EmitterSettings {
            distances: distances.into_settings().unwrap_or_default(),
            attenuation_function: attenuation_function.into_easing(),
            enable_spatialization,
            persist_until_sounds_finish,
        })
    }
}

impl ToSettings for Ref<EmitterDistances> {
    type Settings = kira::spatial::emitter::EmitterDistances;
    fn into_settings(self) -> Option<Self::Settings> {
        if self.is_null() {
            return None;
        }
        let EmitterDistances {
            min_distance,
            max_distance,
        } = unsafe { self.fields() }?.clone();
        Some(kira::spatial::emitter::EmitterDistances {
            min_distance,
            max_distance,
        })
    }
}

/// Interop type for [Ext.reds](https://github.com/cyb3rpsych0s1s/audioware/blob/main/audioware/reds/Ext.reds).
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct AudioSystemExt {
    base: IScriptable,
}

impl AudioSystemExt {
    const fn is_debug(&self) -> bool {
        cfg!(debug_assertions)
    }
    fn duration(
        &self,
        event_name: CName,
        locale: Opt<LocaleExt>,
        gender: Opt<PlayerGender>,
        total: Opt<bool>,
    ) -> f32 {
        let locale = match locale.into_option().map(Locale::try_from) {
            None => None,
            Some(Ok(x)) => Some(x),
            Some(Err(x)) => {
                fails!("invalid locale ({x})");
                return -1.;
            }
        };
        Engine::<CpalBackend>::duration(
            event_name,
            locale.unwrap_or_default(),
            gender.into_option().unwrap_or_default(),
            total.into_option().unwrap_or_default(),
        )
    }
    fn semantic_version(&self) -> StaticArray<u16, 5> {
        StaticArray::from(AUDIOWARE_VERSION)
    }
}

unsafe impl ScriptClass for AudioSystemExt {
    type Kind = Native;
    const NAME: &'static str = "AudioSystemExt";
}

pub trait ExtCommand {
    fn play(
        &self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    );
    fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName);
    fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    );
    /// Play sound on audio emitter with optional [tween][Tween].
    fn play_on_emitter(
        &self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        ext: Ref<AudioSettingsExt>,
    );
    fn stop_on_emitter(
        &self,
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    );
    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_ext: Ref<AudioSettingsExt>,
    );
}

impl ExtCommand for AudioSystemExt {
    fn play(
        &self,
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        ext: Ref<AudioSettingsExt>,
    ) {
        queue::send(Command::Play {
            sound_name,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            line_type: line_type.into_option(),
            ext: ext.into_settings(),
        });
    }

    fn stop(
        &self,
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        queue::send(Command::Stop {
            event_name,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            tween: tween.into_tween(),
        });
    }

    fn play_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        ext: Ref<AudioSettingsExt>,
    ) {
        queue::send(Command::PlayOnEmitter {
            event_name,
            entity_id,
            emitter_name,
            ext: ext.into_settings(),
        });
    }

    fn stop_on_emitter(
        &self,
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        queue::send(Command::StopOnEmitter {
            event_name,
            entity_id,
            emitter_name,
            tween: tween.into_tween(),
        });
    }

    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_ext: Ref<AudioSettingsExt>,
    ) {
        queue::send(Command::Switch {
            switch_name,
            switch_value,
            entity_id: entity_id.into_option(),
            emitter_name: emitter_name.into_option(),
            switch_name_tween: switch_name_tween.into_tween(),
            switch_value_settings: switch_value_ext.into_settings(),
        });
    }

    fn play_over_the_phone(&self, event_name: CName, emitter_name: CName, gender: CName) {
        match PlayerGender::try_from(gender) {
            Ok(gender) => {
                queue::send(Command::PlayOverThePhone {
                    event_name,
                    emitter_name,
                    gender,
                });
            }
            Err(e) => {
                warns!("invalid gender: {e}");
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct DummyLol {
    pub base: IScriptable,
}
unsafe impl ScriptClass for DummyLol {
    type Kind = Native;
    const NAME: &'static str = "Audioware.DummyLol";
}
impl Drop for DummyLol {
    fn drop(&mut self) {
        lifecycle!("drop DummyLol");
    }
}
impl DummyLol {
    pub fn hi(&self) {
        lifecycle!("Hi from DummyLol");
    }
}

#[cfg(feature = "hot-reload")]
mod debug {
    pub trait HotReload {
        fn hot_reload();
    }

    impl HotReload for crate::Audioware {
        fn hot_reload() {
            crate::queue::notify(crate::abi::Lifecycle::HotReload);
        }
    }
}
