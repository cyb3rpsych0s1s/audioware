use audioware_bank::{Banks, Id};
use audioware_manifest::{PlayerGender, ScnDialogLineType, Source, SpokenLocale, WrittenLocale};
use kira::{OutputDestination, Volume};
use modulators::{
    CarRadioVolume, DialogueVolume, MusicVolume, Parameter, RadioportVolume, ReverbMix, SfxVolume,
};
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Opt, Ref},
    PluginOps,
};

use crate::{
    error::Error,
    macros::{ok_or_return, some_or_return},
    states::State,
    types::{
        propagate_subtitles, AsAudioSystem, AsGameInstance, AsGameObject, EmitterSettings,
        GameObject, LocalizationPackage, Subtitle, ToTween, Tween,
    },
    Audioware,
};

mod effects;
mod eq;
mod id;
mod manager;
mod modulators;
mod scene;
mod tracks;

pub use effects::IMMEDIATELY;
pub use eq::EqPass;
pub use eq::Preset;
pub use manager::Manage;
pub use manager::Manager;
pub use scene::Scene;
pub use tracks::Tracks;

pub struct Engine;

impl Engine {
    pub(crate) fn setup() -> Result<(), Error> {
        let mut manager = Manager::try_lock()?;
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager, Tracks::get())?;
        Ok(())
    }
    pub fn define_subtitles(package: Ref<LocalizationPackage>) {
        let written = WrittenLocale::get();
        let subtitles = Banks::subtitles(written);
        for (key, (value_f, value_m)) in subtitles.iter() {
            package.subtitle(key.as_str(), value_f.as_str(), value_m.as_str());
        }
    }
    pub fn supported_languages() -> Vec<CName> {
        Banks::languages().into_iter().map(|x| x.into()).collect()
    }
    pub fn shutdown() {
        if let Err(e) = Manager::clear_tracks(None) {
            log::error!(Audioware::env(), "couldn't clear tracks on manager: {e}");
        }
        if let Err(e) = Scene::clear_emitters() {
            log::error!(Audioware::env(), "couldn't clear emitters in scene: {e}");
        }
    }
    /// Register an audio emitter to spatial scene.
    ///
    /// ⚠️ Returns `true` if already registered.
    pub fn register_emitter(
        entity_id: EntityId,
        emitter_name: Opt<CName>,
        emitter_settings: Opt<EmitterSettings>,
    ) -> bool {
        if let Err(e) = Scene::register_emitter(
            entity_id,
            emitter_name.into_option(),
            emitter_settings.into_option().map(Into::into),
        ) {
            log::error!(Audioware::env(), "couldn't register emitter to scene: {e}");
            return false;
        }
        true
    }
    /// Unregister an audio emitter from spatial scene.
    ///
    /// ⚠️ Returns `true` if already unregistered (or never registered).
    pub fn unregister_emitter(entity_id: EntityId) -> bool {
        if let Err(e) = Scene::unregister_emitter(&entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't unregister emitter from scene: {e}"
            );
            return false;
        }
        true
    }
    pub fn is_registered_emitter(entity_id: EntityId) -> bool {
        Scene::is_registered_emitter(&entity_id)
    }
    pub fn emitters_count() -> i32 {
        let count = Scene::emitters_count();
        if let Err(e) = count {
            log::error!(Audioware::env(), "couldn't count emitters in scene: {e}");
            return -1;
        }
        count.unwrap() as i32
    }
    pub fn sync_emitters() {
        if let Err(e) = Scene::sync_emitters() {
            log::error!(Audioware::env(), "couldn't sync emitters on scene: {e}");
        }
    }
    pub fn sync_listener() {
        if let Err(e) = Scene::sync_listener() {
            log::error!(Audioware::env(), "couldn't sync listener on scene: {e}");
        }
    }
    pub fn play_over_the_phone(event_name: CName, emitter_name: CName, gender: CName) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let gender = ok_or_return!(PlayerGender::try_from(gender), "Play over the phone");
        let id = ok_or_return!(
            Banks::try_get(&event_name, &spoken, Some(&gender)),
            "Unable to get sound ID"
        );
        let _duration = ok_or_return!(
            Manager::play_and_store(
                &mut manager,
                id,
                None,
                Some(emitter_name),
                Some(Tracks::holocall_destination()),
                None
            ),
            "Unable to store sound handle"
        );
        // TODO: handle convo?
    }
    /// play sound
    pub fn play(
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        tween: Ref<Tween>,
    ) {
        let mut manager = ok_or_return!(Manager::try_lock(), "Unable to get audio manager");
        let spoken = SpokenLocale::get();
        let gender = PlayerGender::get();
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();
        let id = ok_or_return!(
            Banks::try_get(&sound_name, &spoken, gender.as_ref()),
            "Unable to get sound ID"
        );

        let tween = tween.into_tween();
        let duration = ok_or_return!(
            Manager::play_and_store(&mut manager, id, entity_id, emitter_name, None, tween),
            "Unable to store sound handle"
        );
        if let (Some(entity_id), Some(emitter_name)) = (entity_id, emitter_name) {
            propagate_subtitles(
                sound_name,
                entity_id,
                emitter_name,
                line_type.unwrap_or_default(),
                duration,
            )
        }
    }
    pub fn stop(
        event_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        tween: Ref<Tween>,
    ) {
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();
        let tween = tween.into_tween();
        log::info!(
            Audioware::env(),
            "stop called: {entity_id:?} {emitter_name:?} {tween:?}"
        );
        if let Err(e) = Manager::stop_by(
            &event_name,
            entity_id.as_ref(),
            emitter_name.as_ref(),
            tween,
        ) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn pause(tween: Ref<Tween>) {
        if let Err(e) = Manager::pause(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn resume(tween: Ref<Tween>) {
        if let Err(e) = Manager::resume(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn switch(
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<Tween>,
        switch_value_tween: Ref<Tween>,
    ) {
        let prev = Banks::exists(&switch_name);
        let next = Banks::exists(&switch_value);
        let system = GameInstance::get_audio_system();

        if prev {
            Engine::stop(switch_name, entity_id, emitter_name, switch_name_tween);
        } else {
            system.stop(switch_name, entity_id, emitter_name);
        }

        if next {
            Engine::play(
                switch_value,
                entity_id,
                emitter_name,
                Opt::Default,
                switch_value_tween,
            );
        } else {
            system.play(switch_value, entity_id, emitter_name);
        }
    }
    pub fn play_on_emitter(
        sound_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        let mut manager = ok_or_return!(Manager::try_lock(), "Unable to get audio manager");
        let spoken = SpokenLocale::get();
        let gender = PlayerGender::get();
        let id = ok_or_return!(
            Banks::try_get(&sound_name, &spoken, gender.as_ref()),
            "Unable to get sound ID"
        );
        let destination = some_or_return!(
            Scene::output_destination(&entity_id),
            "Entity is not registered as emitter",
            entity_id
        );
        let tween = tween.into_tween();
        let duration = ok_or_return!(
            Manager::play_and_store(
                &mut manager,
                id,
                Some(entity_id),
                Some(emitter_name),
                Some(destination),
                tween,
            ),
            "Unable to store sound handle"
        );
        propagate_subtitles(
            sound_name,
            entity_id,
            emitter_name,
            ScnDialogLineType::default(),
            duration,
        );
    }
    pub fn stop_on_emitter(
        event_name: CName,
        entity_id: EntityId,
        emitter_name: CName,
        tween: Ref<Tween>,
    ) {
        if let Err(e) = Manager::stop_by(
            &event_name,
            Some(&entity_id),
            Some(&emitter_name),
            tween.into_tween(),
        ) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn set_reverb_mix(value: f32) {
        if !(0. ..=1.).contains(&value) {
            log::error!(
                Audioware::env(),
                "reverb mix must be between 0. and 1. (inclusive)"
            );
            return;
        }
        ok_or_return!(
            ReverbMix::update(value, IMMEDIATELY),
            "Unable to set reverb mix"
        );
    }
    pub fn set_preset(value: Preset) {
        let tracks = Tracks::get();
        let mut eq = ok_or_return!(tracks.ambience.try_eq(), "Unable to set EQ preset");
        eq.set_preset(value);
    }
    pub fn set_volume(setting: CName, value: f64) {
        if !(0.0..=100.0).contains(&value) {
            log::error!(Audioware::env(), "Volume must be between 0. and 100.");
            return;
        }
        let mut manager = ok_or_return!(Manager::try_lock(), "Unable to get audio manager");
        match setting.as_str() {
            "MasterVolume" => manager
                .main_track()
                .set_volume(Volume::Amplitude(value / 100.), IMMEDIATELY),
            "SfxVolume" => {
                ok_or_return!(
                    SfxVolume::update(value, IMMEDIATELY),
                    "Unable to set SfxVolume"
                );
            }
            "DialogueVolume" => {
                ok_or_return!(
                    DialogueVolume::update(value, IMMEDIATELY),
                    "Unable to set DialogueVolume"
                );
            }
            "MusicVolume" => {
                ok_or_return!(
                    MusicVolume::update(value, IMMEDIATELY),
                    "Unable to set MusicVolume"
                );
            }
            "CarRadioVolume" => {
                ok_or_return!(
                    CarRadioVolume::update(value, IMMEDIATELY),
                    "Unable to set CarRadioVolume"
                );
            }
            "RadioportVolume" => {
                ok_or_return!(
                    RadioportVolume::update(value, IMMEDIATELY),
                    "Unable to set RadioportVolume"
                );
            }
            _ => {
                log::error!(Audioware::env(), "Unknown setting: {setting}");
            }
        };
    }
}

pub trait ToOutputDestination {
    fn output_destination(&self) -> OutputDestination;
}

pub enum Context<'a> {
    Unknown { id: &'a Id },
    Entity { id: &'a Id, entity_id: &'a EntityId },
}
impl<'a> Context<'a> {
    pub fn new(id: &'a Id, entity_id: Option<&'a EntityId>) -> Self {
        match entity_id {
            Some(entity_id) => Self::Entity { id, entity_id },
            None => Self::Unknown { id },
        }
    }
}

impl ToOutputDestination for Id {
    fn output_destination(&self) -> OutputDestination {
        match self {
            Id::OnDemand(_, source) | Id::InMemory(_, source) => match source {
                Source::Sfx => (&Tracks::get().sfx).into(),
                Source::Ono => (&Tracks::get().sfx).into(),
                Source::Voices => (&Tracks::get().dialogue).into(),
                Source::Playlist => (&Tracks::get().radioport).into(),
                Source::Music => (&Tracks::get().music).into(),
                Source::Jingle => (&Tracks::get().car_radio).into(),
            },
        }
    }
}

impl<'a> ToOutputDestination for Context<'a> {
    fn output_destination(&self) -> OutputDestination {
        match self {
            Context::Unknown { id } => id.output_destination(),
            Context::Entity { id, entity_id } => {
                let game = GameInstance::new();
                let entity = GameInstance::find_entity_by_id(game, **entity_id);
                if let Some(go) = entity.cast::<GameObject>() {
                    if go.is_player() {
                        match id {
                            Id::OnDemand(_, source) | Id::InMemory(_, source) => {
                                if let Some(destination) =
                                    Tracks::get().v.output_destination(source)
                                {
                                    return destination;
                                }
                            }
                        };
                    }
                }
                id.output_destination()
            }
        }
    }
}
