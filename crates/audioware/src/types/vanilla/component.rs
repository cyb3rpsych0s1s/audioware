use debug_ignore::DebugIgnore;
use red4ext_rs::{
    NativeRepr, RttiSystem, ScriptClass,
    class_kind::Native,
    types::{CName, Cruid, IScriptable, Method, RedArray, Ref, WeakRef},
};
use std::mem;

use crate::{AudEventStruct, AudParameter, AudSwitch, Quaternion, RedTagList, Vector4};

use super::{Entity, WorldTransform};

#[derive(Debug)]
#[repr(C)]
pub struct IComponent {
    pub base: IScriptable,
    pub name: CName, // 0x40
    pub _padding0: [u8; 0x18],
    pub id: Cruid, // 0x60
    pub _padding1: [u8; 0x23],
    pub is_enabled: bool,    // 0x8B
    pub is_replicable: bool, // 0x8C
}

unsafe impl ScriptClass for IComponent {
    const NAME: &'static str = "entIComponent";
    type Kind = Native;
}

impl AsRef<IScriptable> for IComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

pub trait AsIComponent {
    fn get_entity(&self) -> WeakRef<Entity>;
}

impl AsIComponent for Ref<IComponent> {
    fn get_entity(&self) -> WeakRef<Entity> {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(IComponent::NAME)).unwrap();
        let method = cls.get_method(CName::new("GetEntity")).ok().unwrap();
        method
            .as_function()
            .execute::<_, WeakRef<Entity>>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}

impl AsIComponent for Ref<IPlacedComponent> {
    fn get_entity(&self) -> WeakRef<Entity> {
        unsafe { mem::transmute::<&Ref<IPlacedComponent>, &Ref<IComponent>>(self) }.get_entity()
    }
}

pub trait AsIPlacedComponent {
    fn get_local_orientation(&self) -> Quaternion;
    fn get_local_position(&self) -> Vector4;
}

#[derive(Debug)]
#[repr(C)]
pub struct IPlacedComponent {
    pub base: IComponent,                                // 0x0
    pub parent_transform: DebugIgnore<Ref<IScriptable>>, // 0x90
    pub _padding2: [u8; 0x20],
    pub local_transform: WorldTransform, // 0xC0
    pub _padding3: [u8; 0x40],
}

unsafe impl ScriptClass for IPlacedComponent {
    const NAME: &'static str = "entIPlacedComponent";
    type Kind = Native;
}

impl AsRef<IComponent> for IPlacedComponent {
    #[inline]
    fn as_ref(&self) -> &IComponent {
        &self.base
    }
}

impl AsRef<IScriptable> for IPlacedComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

impl AsIPlacedComponent for Ref<IPlacedComponent> {
    fn get_local_orientation(&self) -> Quaternion {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls
            .get_method(CName::new("GetLocalOrientation"))
            .ok()
            .unwrap();
        match unsafe { self.instance() } {
            Some(x) => method
                .as_function()
                .execute::<_, Quaternion>(Some(x.as_ref()), ())
                .unwrap(),
            _ => Quaternion::default(),
        }
    }

    fn get_local_position(&self) -> Vector4 {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(Entity::NAME)).unwrap();
        let method: &Method = cls.get_method(CName::new("GetLocalPosition")).ok().unwrap();
        match unsafe { self.instance() } {
            Some(x) => method
                .as_function()
                .execute::<_, Vector4>(Some(x.as_ref()), ())
                .unwrap(),
            _ => Vector4::default(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioEmitterComponent {
    base: IPlacedComponent,                     // 0
    unk120: [u8; 0x138 - 0x120],                // 120
    pub update_distance: f32,                   // 138
    unk13c: [u8; 0x140 - 0x13C],                // 13C
    pub emitter_name: CName,                    // 140
    pub emitter_type: EntityEmitterContextType, // 148
    unk14c: [u8; 0x150 - 0x14C],                // 14C
    pub on_attach: AudioSyncs,                  // 150
    pub on_detach: AudioSyncs,                  // 190
    pub tags: RedArray<CName>,                  // 1D0
    pub tag_list: DebugIgnore<RedTagList>,      // 1E0
    pub emitter_metadata_name: CName,           // 1F0
    unk1f8: [u8; 0x200 - 0x1F8],                // 1F8
}

unsafe impl ScriptClass for AudioEmitterComponent {
    type Kind = Native;
    const NAME: &'static str = "gameAudioEmitterComponent";
}

impl AsRef<IScriptable> for AudioEmitterComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum EntityEmitterContextType {
    #[default]
    EntityEmitter = 0,
    IndependentEmitter = 1,
    RadioEmitter = 2,
    PermanentObjectEmitter = 3,
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioSyncs {
    switch_events: RedArray<AudSwitch>,       // 00
    play_events: RedArray<AudEventStruct>,    // 10
    stop_events: RedArray<AudEventStruct>,    // 20
    parameter_events: RedArray<AudParameter>, // 30
}

unsafe impl NativeRepr for AudioSyncs {
    const NAME: &'static str = "gameAudioSyncs";
}

const PADDING_1E0: usize = 0x1E8 - 0x1E0;
const PADDING_288: usize = 0x2AC - 0x288;
const PADDING_2B4: usize = 0x320 - 0x2B4;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct CameraComponent {
    base: BaseCameraComponent,                     // 0
    unk1e0: [u8; PADDING_1E0],                     // 1E0
    anim_param_fov_override_weight: CName,         // 1E8
    anim_param_fov_override_value: CName,          // 1F0
    anim_param_zoom_override_weight: CName,        // 1F8
    anim_param_zoom_override_value: CName,         // 200
    anim_param_zoom_weapon_override_weight: CName, // 208
    anim_param_zoom_weapon_override_value: CName,  // 210
    anim_paramdof_intensity: CName,                // 218
    anim_paramdof_near_blur: CName,                // 220
    anim_paramdof_near_focus: CName,               // 228
    anim_paramdof_far_blur: CName,                 // 230
    anim_paramdof_far_focus: CName,                // 238
    anim_param_weapon_near_plane_cm: CName,        // 240
    anim_param_weapon_far_plane_cm: CName,         // 248
    anim_param_weapon_edges_sharpness: CName,      // 250
    anim_param_weapon_vignette_intensity: CName,   // 258
    anim_param_weapon_vignette_radius: CName,      // 260
    anim_param_weapon_vignette_circular: CName,    // 268
    anim_param_weapon_blur_intensity: CName,       // 270
    zoom_override_weight: f32,                     // 278
    zoom_override_value: f32,                      // 27C
    zoom_weapon_override_weight: f32,              // 280
    zoom_weapon_override_value: f32,               // 284
    unk288: [u8; PADDING_288],                     // 288
    fov_override_weight: f32,                      // 2AC
    fov_override_value: f32,                       // 2B0
    unk2b4: [u8; PADDING_2B4],                     // 2B4
}

unsafe impl ScriptClass for CameraComponent {
    type Kind = Native;
    const NAME: &'static str = "gameCameraComponent";
}

impl AsRef<IScriptable> for CameraComponent {
    fn as_ref(&self) -> &IScriptable {
        &self.base.base.base.base
    }
}

const PADDING_120: usize = 0x128 - 0x120;
const PADDING_12C: usize = 0x15C - 0x12C;
const PADDING_160: usize = 0x168 - 0x160;
const PADDING_174: usize = 0x178 - 0x174;
const PADDING_194: usize = 0x1E0 - 0x194;

#[derive(Debug)]
#[repr(C)]
pub struct BaseCameraComponent {
    base: IPlacedComponent,
    unk120: [u8; PADDING_120],        // 120
    fov: f32,                         // 128
    unk12c: [u8; PADDING_12C],        // 12C
    zoom: f32,                        // 15C
    unk160: [u8; PADDING_160],        // 160
    near_plane_override: f32,         // 168
    far_plane_override: f32,          // 16C
    motion_blur_scale: f32,           // 170
    unk174: [u8; PADDING_174],        // 174
    weapon_plane: SWeaponPlaneParams, // 178
    unk194: [u8; PADDING_194],        // 194
}

unsafe impl ScriptClass for BaseCameraComponent {
    type Kind = Native;
    const NAME: &'static str = "entBaseCameraComponent";
}

const PADDING_04: usize = 0x18 - 0x4;

#[derive(Debug)]
#[repr(C)]
pub struct SWeaponPlaneParams {
    weapon_near_plane_cm: f32, // 00
    unk04: [u8; PADDING_04],   // 4
    blur_intensity: f32,       // 18
}

unsafe impl NativeRepr for SWeaponPlaneParams {
    const NAME: &'static str = "SWeaponPlaneParams";
}
