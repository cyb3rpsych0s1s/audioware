use red4ext_rs::{
    RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, EntityId, IScriptable, Method, Opt, Ref},
};

#[repr(C)]
pub struct AudioSystem {
    pub base: IScriptable,
    pub _padding0: [u8; 0x3E0],
}

unsafe impl ScriptClass for AudioSystem {
    const NAME: &'static str = "gameGameAudioSystem";
    type Kind = Native;
}

impl AsRef<IScriptable> for AudioSystem {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsAudioSystem {
    fn play(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>);
    fn stop(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>);
    fn switch(
        &self,
        switch_name: CName,
        switch_value: CName,
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    );
}

impl AsAudioSystem for Ref<AudioSystem> {
    fn play(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
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

    fn stop(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
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
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
    ) {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(AudioSystem::NAME)).unwrap();
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
