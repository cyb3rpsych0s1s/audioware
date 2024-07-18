use red4ext_rs::{
    types::{CName, EntityId, IScriptable, Method, Native, Opt, Ref, ScriptClass},
    NativeRepr, RttiSystem,
};

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

impl GameAudioSystem for Ref<AudioSystem> {
    fn play(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
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

    fn stop(&self, event_name: CName, entity_id: Opt<EntityId>, emitter_name: Opt<CName>) {
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
        entity_id: Opt<EntityId>,
        emitter_name: Opt<CName>,
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(C, align(16))]
pub struct Vector4 {
    pub x: f32, // 0x0
    pub y: f32, // 0x4
    pub z: f32, // 0x8
    pub w: f32, // 0xC
}

unsafe impl NativeRepr for Vector4 {
    const NAME: &'static str = "Vector4";
}
