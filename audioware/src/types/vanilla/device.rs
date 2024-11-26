use red4ext_rs::{
    class_kind::{Native, Scripted},
    types::{
        CName, Cruid, EntityId, IScriptable, ISerializable, NodeRef, RaRef, RedArray, RedString,
        Ref, TweakDbId, WeakRef,
    },
    NativeRepr, ScriptClass,
};

use super::{CallbackHandle, DelayId, GameObject, InteractionChoice, Vector4};

#[repr(C)]
pub struct Device {
    pub _padding0: [u8; 0xF0],
    pub controller: Ref<IScriptable>,           // 0xF0
    pub was_visible: bool,                      // 0x100
    pub is_visible: bool,                       // 0x101
    pub controller_type_name: CName,            // 0x108
    pub device_state: EDeviceStatus,            // 0x110
    pub ui_component: WeakRef<IScriptable>,     // 0x118
    pub screen_definition: SuiScreenDefinition, // 0x128
    pub is_u_idirty: bool,                      // 0x138
    pub on_input_hint_manager_initialized_changed_callback: Ref<CallbackHandle>, // 0x140
    pub personal_link_component: Ref<IScriptable>, // 0x150
    pub durability_type: EDeviceDurabilityType, // 0x160
    pub disassemblable_component: Ref<IScriptable>, // 0x168
    pub localization: Ref<IScriptable>,         // 0x178
    pub i_kslot_component: Ref<IScriptable>,    // 0x188
    pub slot_component: Ref<IScriptable>,       // 0x198
    pub is_initialized: bool,                   // 0x1A8
    pub is_inside_logic_area: bool,             // 0x1A9
    pub camera_component: Ref<IScriptable>,     // 0x1B0
    pub camera_zoom_component: Ref<IScriptable>, // 0x1C0
    pub camera_zoom_active: bool,               // 0x1D0
    pub toggle_zoom_interaction_workspot: Ref<IScriptable>, // 0x1D8
    pub zoom_ui_listener_id: Ref<CallbackHandle>, // 0x1E8
    pub zoom_state_machine_listener_id: Ref<CallbackHandle>, // 0x1F8
    pub advance_interaction_state_resolve_delay_id: DelayId, // 0x208
    pub active_status_effect: TweakDbId,        // 0x210
    pub active_program_to_upload_on_npc: TweakDbId, // 0x218
    pub is_qhack_upload_in_progerss: bool,      // 0x220
    pub scanning_tweak_db_record: TweakDbId,    // 0x224
    pub update_running: bool,                   // 0x22C
    pub update_id: DelayId,                     // 0x230
    pub delayed_update_device_state_id: DelayId, // 0x238
    pub blackboard: Ref<IScriptable>,           // 0x240
    pub current_player_target_callback_id: Ref<CallbackHandle>, // 0x250
    pub was_looked_at_last: bool,               // 0x260
    pub last_ping_source_id: EntityId,          // 0x268
    pub network_grid_beam_fx: FxResource,       // 0x270
    pub fx_resource_mapper: Ref<IScriptable>,   // 0x278
    pub effect_visualization: Ref<IScriptable>, // 0x288
    pub resource_library_component: Ref<IScriptable>, // 0x298
    pub gameplay_role_component: Ref<IScriptable>, // 0x2A8
    pub personal_link_hack_send: bool,          // 0x2B8
    pub personal_link_failsafe_id: DelayId,     // 0x2BC
    pub was_animation_fast_forwarded: bool,     // 0x2C4
    pub was_engineering_skillcheck_triggered: bool, // 0x2C5
    pub content_scale: TweakDbId,               // 0x2C8
    pub network_grid_beam_offset: Vector4,      // 0x2D0
    pub area_effects_data: RedArray<SAreaEffectData>, // 0x2E0
    pub area_effects_in_focus_mode: RedArray<SAreaEffectTargetData>, // 0x2F0
    pub debug_options: DebuggerProperties,      // 0x300
    pub currently_uploading_action: WeakRef<ScriptableDeviceAction>, // 0x328
    pub workspot_activator: WeakRef<GameObject>, // 0x338
}

unsafe impl ScriptClass for Device {
    const NAME: &'static str = "Device";
    type Kind = Scripted;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum EDeviceStatus {
    Disabled = -2,
    Unpowered = -1,
    Off = 0,
    On = 1,
    Invalid = 2,
}

unsafe impl NativeRepr for EDeviceStatus {
    const NAME: &'static str = "EDeviceStatus";
}

#[repr(C, align(4))]
pub struct SuiScreenDefinition {
    pub _padding0: [u8; 0x10],
}

unsafe impl NativeRepr for SuiScreenDefinition {
    const NAME: &'static str = "SUIScreenDefinition";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EDeviceDurabilityType {
    Invulnerable = 0,
    Indestructible = 1,
    Destructible = 2,
}

unsafe impl NativeRepr for EDeviceDurabilityType {
    const NAME: &'static str = "EDeviceDurabilityType";
}

#[repr(C, align(8))]
pub struct FxResource {
    pub effect: RaRef<WorldEffect>, // 0x0
}

unsafe impl NativeRepr for FxResource {
    const NAME: &'static str = "gameFxResource";
}

#[repr(C)]
pub struct WorldEffect {
    pub base: ISerializable,
    pub _padding0: [u8; 0x8],
    pub cooking_platform: ECookingPlatform,     // 0x38
    pub name: CName,                            // 0x40
    pub length: f32,                            // 0x48
    pub track_root: Ref<EffectTrackGroup>,      // 0x50
    pub events: RedArray<Ref<EffectTrackItem>>, // 0x60
    pub effect_loops: RedArray<EffectLoopData>, // 0x70
    pub input_parameter_names: RedArray<CName>, // 0x80
}

unsafe impl ScriptClass for WorldEffect {
    const NAME: &'static str = "worldEffect";
    type Kind = Native;
}

impl AsRef<ISerializable> for WorldEffect {
    #[inline]
    fn as_ref(&self) -> &ISerializable {
        &self.base
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ECookingPlatform {
    PlatformNone = 0,
    PlatformPc = 1,
    PlatformXboxOne = 2,
    PlatformPs4 = 3,
    PlatformPs5 = 4,
    PlatformXsx = 5,
    PlatformWindowsServer = 6,
    PlatformLinuxServer = 7,
    PlatformGgp = 8,
}

unsafe impl NativeRepr for ECookingPlatform {
    const NAME: &'static str = "ECookingPlatform";
}

#[repr(C)]
pub struct EffectTrackGroup {
    pub base: ISerializable,
    pub tracks: RedArray<Ref<EffectTrackBase>>, // 0x30
    pub component_name: CName,                  // 0x40
}

unsafe impl ScriptClass for EffectTrackGroup {
    const NAME: &'static str = "effectTrackGroup";
    type Kind = Native;
}

impl AsRef<ISerializable> for EffectTrackGroup {
    #[inline]
    fn as_ref(&self) -> &ISerializable {
        &self.base
    }
}

#[repr(C)]
pub struct EffectTrackBase {
    pub base: ISerializable,
}

unsafe impl ScriptClass for EffectTrackBase {
    const NAME: &'static str = "effectTrackBase";
    type Kind = Native;
}

impl AsRef<ISerializable> for EffectTrackBase {
    #[inline]
    fn as_ref(&self) -> &ISerializable {
        &self.base
    }
}

#[repr(C)]
pub struct EffectTrackItem {
    pub base: ISerializable,
    pub time_begin: f32,    // 0x30
    pub time_duration: f32, // 0x34
    pub ruid: Cruid,        // 0x38
    pub _padding0: [u8; 0x8],
}

unsafe impl ScriptClass for EffectTrackItem {
    const NAME: &'static str = "effectTrackItem";
    type Kind = Native;
}

impl AsRef<ISerializable> for EffectTrackItem {
    #[inline]
    fn as_ref(&self) -> &ISerializable {
        &self.base
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct EffectLoopData {
    pub start_time: f32, // 0x0
    pub end_time: f32,   // 0x4
}

unsafe impl NativeRepr for EffectLoopData {
    const NAME: &'static str = "effectLoopData";
}

#[repr(C, align(8))]
pub struct SAreaEffectData {
    pub _padding0: [u8; 0x88],
}

unsafe impl NativeRepr for SAreaEffectData {
    const NAME: &'static str = "SAreaEffectData";
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct SAreaEffectTargetData {
    pub _padding0: [u8; 0x10],
}

unsafe impl NativeRepr for SAreaEffectTargetData {
    const NAME: &'static str = "SAreaEffectTargetData";
}

#[repr(C, align(8))]
pub struct DebuggerProperties {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for DebuggerProperties {
    const NAME: &'static str = "DebuggerProperties";
}

#[repr(C)]
pub struct ScriptableDeviceAction {
    pub requester_id: EntityId,                          // 0x0
    pub executor: WeakRef<GameObject>,                   // 0x8
    pub proxy_executor: WeakRef<GameObject>,             // 0x18
    pub cost_components: RedArray<WeakRef<IScriptable>>, // 0x28
    pub object_action_id: TweakDbId,                     // 0x38
    pub object_action_record: WeakRef<IScriptable>,      // 0x40
    pub ink_widget_id: TweakDbId,                        // 0x50
    pub interaction_choice: InteractionChoice,           // 0x58
    pub interaction_layer: CName,                        // 0xF8
    pub is_action_rpg_check_dissabled: bool,             // 0x100
    pub can_skip_pay_cost: bool,                         // 0x101
    pub calculated_base_cost: i32,                       // 0x104
    pub device_action_queue: Ref<IScriptable>,           // 0x108
    pub is_action_queueing_used: bool,                   // 0x118
    pub is_queued_action: bool,                          // 0x119
    pub is_inactive: bool,                               // 0x11A
    pub is_target_dead: bool,                            // 0x11B
    pub activation_time_reduction: f32,                  // 0x11C
    pub is_applied_by_monowire: bool,                    // 0x120
    pub prop: Ref<IScriptable>,                          // 0x128
    pub action_widget_package: SActionWidgetPackage,     // 0x138
    pub spiderbot_action_location_override: NodeRef,     // 0x228
    pub duration: f32,                                   // 0x230
    pub can_trigger_stim: bool,                          // 0x234
    pub was_performed_on_owner: bool,                    // 0x235
    pub should_activate_device: bool,                    // 0x236
    pub disable_spread: bool,                            // 0x237
    pub is_quick_hack: bool,                             // 0x238
    pub is_spiderbot_action: bool,                       // 0x239
    pub attached_program: TweakDbId,                     // 0x23C
    pub active_status_effect: TweakDbId,                 // 0x244
    pub interaction_icon_type: TweakDbId,                // 0x24C
    pub has_interaction: bool,                           // 0x254
    pub inactive_reason: RedString,                      // 0x258
    pub widget_style: GamedataComputerUiStyle,           // 0x278
}

unsafe impl ScriptClass for ScriptableDeviceAction {
    const NAME: &'static str = "ScriptableDeviceAction";
    type Kind = Scripted;
}

#[repr(C, align(8))]
pub struct SActionWidgetPackage {
    pub _padding0: [u8; 0xF0],
}

unsafe impl NativeRepr for SActionWidgetPackage {
    const NAME: &'static str = "SActionWidgetPackage";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamedataComputerUiStyle {
    DarkBlue = 0,
    LightBlue = 1,
    Orange = 2,
    Count = 3,
    Invalid = 4,
}

unsafe impl NativeRepr for GamedataComputerUiStyle {
    const NAME: &'static str = "gamedataComputerUIStyle";
}
