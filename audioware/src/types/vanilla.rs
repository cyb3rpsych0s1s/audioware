use red4ext_rs::{
    call,
    types::{CName, EntityId, IScriptable, Method, Native, Ref, ScriptClass},
    NativeRepr, RttiSystem,
};

#[repr(C, align(8))]
#[derive(Default)]
pub struct GameInstance {
    pub _padding0: [u8; 0x18],
}

unsafe impl NativeRepr for GameInstance {
    const NAME: &'static str = "ScriptGameInstance";
}

pub trait IGameInstance {
    fn get_audio_system(game: GameInstance) -> Ref<AudioSystem>;
}

impl IGameInstance for GameInstance {
    fn get_audio_system(game: GameInstance) -> Ref<AudioSystem> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Self::NAME)).unwrap();
        let static_method = cls
            .static_methods()
            .iter()
            .find(|x| x.as_function().name() == CName::new("GetAudioSystem"))
            .unwrap();
        static_method.as_function().execute(None, (game,)).unwrap()
    }
}

#[repr(C)]
pub struct AudioSystem {
    pub base: IScriptable,
    pub _padding0: [u8; 0x3E0],
}

unsafe impl ScriptClass for AudioSystem {
    const CLASS_NAME: &'static str = "gameGameAudioSystem";
    type Kind = Native;
}

impl AsRef<IScriptable> for AudioSystem {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[allow(dead_code)]
pub trait GameAudioSystem {
    fn play(&self, event_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>);
    fn stop(&self, event_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>);
    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    );
}

impl GameAudioSystem for Ref<AudioSystem> {
    fn play(&self, event_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::CLASS_NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Play")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    event_name,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }

    fn stop(&self, event_name: CName, entity_id: Option<EntityId>, emitter_name: Option<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::CLASS_NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Stop")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    event_name,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }

    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Option<EntityId>,
        emitter_name: Option<CName>,
    ) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::CLASS_NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("Switch")).ok().unwrap();
        method
            .as_function()
            .execute::<_, ()>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (
                    switch_name,
                    switch_value,
                    entity_id.unwrap_or_default(),
                    emitter_name.unwrap_or_default(),
                ),
            )
            .unwrap();
    }
}

pub fn get_game_instance() -> GameInstance {
    call!("GetGameInstance"() -> GameInstance).unwrap_or_default()
}
