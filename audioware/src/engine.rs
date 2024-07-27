use audioware_bank::Banks;
use audioware_manifest::{PlayerGender, ScnDialogLineType, SpokenLocale, WrittenLocale};
use id::HandleId;
use red4ext_rs::{
    log,
    types::{CName, EntityId, GameInstance, Opt, Ref},
    PluginOps,
};

use crate::{
    error::Error,
    states::State,
    types::{
        propagate_subtitles, AsAudioSystem, AsGameInstance, AudiowareTween, LocalizationPackage,
        Subtitle, ToTween,
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
pub use manager::StaticStorage;
pub use manager::StreamStorage;
pub use scene::Scene;
pub use tracks::Tracks;

pub struct Engine;

impl Engine {
    pub(crate) fn setup() -> Result<(), Error> {
        // SAFETY: initialization order matters
        let mut manager = Manager::try_lock()?;
        Tracks::setup(&mut manager)?;
        Scene::setup(&mut manager, &Tracks::get().v.main)?;
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
    pub fn register_emitter(entity_id: EntityId, emitter_name: Opt<CName>) {
        if let Err(e) = Scene::register_emitter(entity_id, emitter_name.into_option()) {
            log::error!(Audioware::env(), "couldn't register emitter to scene: {e}");
        }
    }
    pub fn unregister_emitter(entity_id: EntityId) {
        if let Err(e) = Scene::unregister_emitter(&entity_id) {
            log::error!(
                Audioware::env(),
                "couldn't unregister emitter from scene: {e}"
            );
        }
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
        let gender = match PlayerGender::try_from(gender) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Play over the phone: {e}");
                return;
            }
        };
        let id = match Banks::try_get(&event_name, &spoken, Some(&gender)) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get sound ID: {e}");
                return;
            }
        };
        #[allow(unused_assignments)]
        let mut duration: f32 = 3.0;
        match Banks::data(id) {
            either::Either::Left(data) => {
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data).unwrap();
                match StaticStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, None, Some(emitter_name)), handle);
                    }
                    Err(e) => {
                        log::error!(Audioware::env(), "Unable to store static sound handle: {e}");
                    }
                }
            }
            either::Either::Right(data) => {
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data).unwrap();
                match StreamStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, None, Some(emitter_name)), handle);
                    }
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "Unable to store streaming sound handle: {e}"
                        );
                    }
                }
            }
        }
        log::info!(
            Audioware::env(),
            "about to call propagate subtitle ({})",
            emitter_name
        );
        propagate_subtitles(
            event_name,
            todo!(),
            emitter_name,
            ScnDialogLineType::Holocall,
            duration,
        );
    }
    /// play sound
    pub fn play(
        sound_name: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        line_type: Opt<ScnDialogLineType>,
        tween: Ref<AudiowareTween>,
    ) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let gender = PlayerGender::get();
        let entity_id = entity_id.into_option();
        let emitter_name = emitter_name.into_option();
        let id = match Banks::try_get(&sound_name, &spoken, gender.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get sound ID: {e}");
                return;
            }
        };
        // TODO: output destination
        let tween = tween.into_tween();
        #[allow(unused_assignments)]
        let mut duration: f32 = 3.0;
        match Banks::data(id) {
            either::Either::Left(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data).unwrap();
                match StaticStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, entity_id, emitter_name), handle);
                    }
                    Err(e) => {
                        log::error!(Audioware::env(), "Unable to store static sound handle: {e}");
                    }
                }
            }
            either::Either::Right(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data).unwrap();
                match StreamStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(HandleId::new(id, entity_id, emitter_name), handle);
                    }
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "Unable to store streaming sound handle: {e}"
                        );
                    }
                }
            }
        }
        log::info!(
            Audioware::env(),
            "about to call propagate subtitle ({:?}, {:?})",
            entity_id,
            emitter_name
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
        tween: Ref<AudiowareTween>,
    ) {
        if let Err(e) = Manager::stop_by(
            &event_name,
            entity_id.into_option().as_ref(),
            emitter_name.into_option().as_ref(),
            tween.into_tween(),
        ) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn pause(tween: Ref<AudiowareTween>) {
        if let Err(e) = Manager::pause(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn resume(tween: Ref<AudiowareTween>) {
        if let Err(e) = Manager::resume(tween.into_tween()) {
            log::error!(Audioware::env(), "{e}");
        }
    }
    pub fn switch(
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
        switch_name_tween: Ref<AudiowareTween>,
        switch_value_tween: Ref<AudiowareTween>,
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
        tween: Ref<AudiowareTween>,
    ) {
        let mut manager = match Manager::try_lock() {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get audio manager: {e}");
                return;
            }
        };
        let spoken = SpokenLocale::get();
        let written = WrittenLocale::get();
        let gender = PlayerGender::get();
        let id = match Banks::try_get(&sound_name, &spoken, gender.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                log::error!(Audioware::env(), "Unable to get sound ID: {e}");
                return;
            }
        };
        let destination = match Scene::output_destination(&entity_id) {
            Some(x) => x,
            None => {
                log::error!(
                    Audioware::env(),
                    "Entity is not registered as emitter: {entity_id:?}"
                );
                return;
            }
        };
        let mut duration: f32 = 3.0;
        let tween = tween.into_tween();
        match Banks::data(id) {
            either::Either::Left(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data.output_destination(destination)).unwrap();
                match StaticStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(
                            HandleId::new(id, Some(entity_id), Some(emitter_name)),
                            handle,
                        );
                    }
                    Err(e) => {
                        log::error!(Audioware::env(), "Unable to store static sound handle: {e}");
                    }
                }
            }
            either::Either::Right(mut data) => {
                if tween.is_some() {
                    data.settings.fade_in_tween = tween;
                }
                duration = data.duration().as_secs_f32();
                let handle = manager.play(data.output_destination(destination)).unwrap();
                match StreamStorage::try_lock() {
                    Ok(mut x) => {
                        x.insert(
                            HandleId::new(id, Some(entity_id), Some(emitter_name)),
                            handle,
                        );
                    }
                    Err(e) => {
                        log::error!(
                            Audioware::env(),
                            "Unable to store streaming sound handle: {e}"
                        );
                    }
                }
            }
        }
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
        tween: Ref<AudiowareTween>,
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
    pub fn set_player_reverb(value: f32) {
        if !(0. ..=1.).contains(&value) {
            log::error!(
                Audioware::env(),
                "reverb must be between 0. and 1. (inclusive)"
            );
            return;
        }
        let tracks = Tracks::get();
        match tracks.reverb.try_lock() {
            Ok(ref mut x) => x.set_volume(kira::Volume::Amplitude(value as f64), IMMEDIATELY),
            Err(e) => log::error!(Audioware::env(), "Unable to set reverb volume: {e}"),
        }
    }
    pub fn set_player_preset(value: Preset) {
        let tracks = Tracks::get();
        match tracks.v.eq.try_lock() {
            Ok(ref mut x) => x.set_preset(value),
            Err(e) => log::error!(Audioware::env(), "Unable to set EQ preset: {e}"),
        }
    }
}
