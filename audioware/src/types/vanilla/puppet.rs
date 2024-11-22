use red4ext_rs::{
    class_kind::{Native, Scripted},
    types::{
        CName, Cruid, EngineTime, EntityId, IScriptable, ItemId, RedArray, Ref, TweakDbId, WeakRef,
    },
    NativeRepr, RttiSystem, ScriptClass,
};

use super::{Entity, GameObject};

const PADDING: usize = std::mem::size_of::<Entity>() - 0xF0;

#[repr(C)]
pub struct PlayerPuppet {
    pub base: ScriptedPuppet,
    pub quick_slots_manager: Ref<IScriptable>,  // 0x420
    pub inspection_component: Ref<IScriptable>, // 0x430
    pub enviro_damage_rcv_component: Ref<IScriptable>, // 0x440
    pub mounted_vehicle: WeakRef<IScriptable>,  // 0x450
    pub vehicle_knockdown_timestamp: f32,       // 0x460
    pub phone: Ref<PlayerPhone>,                // 0x468
    pub fpp_camera_component: Ref<IScriptable>, // 0x478
    pub primary_targeting_component: Ref<IScriptable>, // 0x488
    pub breach_finder_component: Ref<IScriptable>, // 0x498
    pub chase_spawn_component: Ref<IScriptable>, // 0x4A8
    pub is_in_finisher: bool,                   // 0x4B8
    pub debug_visualizer: Ref<IScriptable>,     // 0x4C0
    pub debug_damage_input_rec: Ref<IScriptable>, // 0x4D0
    pub high_damage_threshold: f32,             // 0x4E0
    pub med_damage_threshold: f32,              // 0x4E4
    pub low_damage_threshold: f32,              // 0x4E8
    pub melee_high_damage_threshold: f32,       // 0x4EC
    pub melee_med_damage_threshold: f32,        // 0x4F0
    pub melee_low_damage_threshold: f32,        // 0x4F4
    pub explosion_high_damage_threshold: f32,   // 0x4F8
    pub explosion_med_damage_threshold: f32,    // 0x4FC
    pub explosion_low_damage_threshold: f32,    // 0x500
    pub effect_time_stamp: f32,                 // 0x504
    pub cur_inventory_weight: f32,              // 0x508
    pub health_vfx_blackboard: Ref<IScriptable>, // 0x510
    pub laser_targetting_vfx_blackboard: Ref<IScriptable>, // 0x520
    pub item_log_blackboard: WeakRef<IScriptable>, // 0x530
    pub interaction_data_listener: Ref<CallbackHandle>, // 0x540
    pub popup_is_modal_listener: Ref<CallbackHandle>, // 0x550
    pub ui_vendor_context_listener: Ref<CallbackHandle>, // 0x560
    pub ui_radial_contextistener: Ref<CallbackHandle>, // 0x570
    pub contacts_active_listener: Ref<CallbackHandle>, // 0x580
    pub sms_messenger_active_listener: Ref<CallbackHandle>, // 0x590
    pub current_visible_target_listener: Ref<CallbackHandle>, // 0x5A0
    pub last_scan_target: WeakRef<GameObject>,  // 0x5B0
    pub melee_select_input_processed: bool,     // 0x5C0
    pub waiting_for_delay_event: bool,          // 0x5C1
    pub randomized_time: f32,                   // 0x5C4
    pub is_resetting: bool,                     // 0x5C8
    pub delay_event_id: DelayId,                // 0x5CC
    pub reset_tick_id: DelayId,                 // 0x5D4
    pub katana_anim_progression: f32,           // 0x5DC
    pub cover_modifier_active: bool,            // 0x5E0
    pub workspot_damage_reduction_active: bool, // 0x5E1
    pub workspot_visibility_reduction_active: bool, // 0x5E2
    pub current_player_workspot_tags: RedArray<CName>, // 0x5E8
    pub incapacitated: bool,                    // 0x5F8
    pub remote_mappin_id: NewMappinId,          // 0x600
    pub cpo_mission_data_state: Ref<IScriptable>, // 0x608
    pub cpo_mission_data_bb_id: Ref<CallbackHandle>, // 0x618
    pub visibility_listener: Ref<IScriptable>,  // 0x628
    pub second_heart_listener: Ref<IScriptable>, // 0x638
    pub armor_stat_listener: Ref<IScriptable>,  // 0x648
    pub health_stat_listener: Ref<IScriptable>, // 0x658
    pub oxygen_stat_listener: Ref<IScriptable>, // 0x668
    pub aim_assist_listener: Ref<IScriptable>,  // 0x678
    pub auto_reveal_listener: Ref<IScriptable>, // 0x688
    pub all_stats_listener: Ref<IScriptable>,   // 0x698
    pub right_hand_attachment_slot_listener: Ref<IScriptable>, // 0x6A8
    pub healing_items_charge_stat_listener: Ref<IScriptable>, // 0x6B8
    pub grenades_charge_stat_listener: Ref<IScriptable>, // 0x6C8
    pub projectile_launcher_charge_stat_listener: Ref<IScriptable>, // 0x6D8
    pub optical_camo_charge_stat_listener: Ref<IScriptable>, // 0x6E8
    pub overclock_charge_listener: Ref<IScriptable>, // 0x6F8
    pub accessibility_controls_listener: Ref<IScriptable>, // 0x708
    pub is_talking_on_phone: bool,              // 0x718
    pub data_damage_update_id: DelayId,         // 0x71C
    pub player_attached_callback_id: u32,       // 0x724
    pub player_detached_callback_id: u32,       // 0x728
    pub callback_handles: RedArray<Ref<CallbackHandle>>, // 0x730
    pub number_of_combatants: i32,              // 0x740
    pub equipment_mesh_overlay_effect_name: CName, // 0x748
    pub equipment_mesh_overlay_effect_tag: CName, // 0x750
    pub equipment_mesh_overlay_slots: RedArray<TweakDbId>, // 0x758
    pub cover_visibility_perk_blocked: bool,    // 0x768
    pub behind_cover: bool,                     // 0x769
    pub in_combat: bool,                        // 0x76A
    pub is_being_revealed: bool,                // 0x76B
    pub has_been_detected: bool,                // 0x76C
    pub in_crouch: bool,                        // 0x76D
    pub has_kiroshi_optics_fragment: bool,      // 0x76E
    pub doing_quick_melee: bool,                // 0x76F
    pub vehicle_state: GamePsmVehicle,          // 0x770
    pub in_mounted_weapon_vehicle: bool,        // 0x774
    pub in_driver_combat_tpp: bool,             // 0x775
    pub driver_combat_weapon_type: [u8; 0x4],   // 0x776
    pub is_aiming: bool,                        // 0x77A
    pub focus_mode_active: bool,                // 0x77B
    pub custom_fast_forward_possible: bool,     // 0x77C
    pub equipped_right_hand_weapon: WeakRef<IScriptable>, // 0x780
    pub aim_assist_update_queued: bool,         // 0x790
    pub locomotion_state: i32,                  // 0x794
    pub left_hand_cyberware_state: i32,         // 0x798
    pub melee_weapon_state: i32,                // 0x79C
    pub weapon_zoom_level: f32,                 // 0x7A0
    pub controlling_device_id: EntityId,        // 0x7A8
    pub gunshot_range: f32,                     // 0x7B0
    pub explosion_range: f32,                   // 0x7B4
    pub is_in_body_slam: bool,                  // 0x7B8
    pub combat_gadget_state: i32,               // 0x7BC
    pub scene_tier: GameplayTier,               // 0x7C0
    pub next_buffer_modifier: i32,              // 0x7C4
    pub attacking_netrunner_id: EntityId,       // 0x7C8
    pub npc_death_instigator: WeakRef<IScriptable>, // 0x7D0
    pub best_targetting_weapon: WeakRef<IScriptable>, // 0x7E0
    pub best_targetting_dot: f32,               // 0x7F0
    pub targetting_enemies: i32,                // 0x7F4
    pub is_aiming_at_friendly: bool,            // 0x7F8
    pub is_aiming_at_child: bool,               // 0x7F9
    pub distance_from_target_squared: f32,      // 0x7FC
    pub cover_record_id: TweakDbId,             // 0x800
    pub damage_reduction_record_id: TweakDbId,  // 0x808
    pub vis_reduction_record_id: TweakDbId,     // 0x810
    pub last_dmg_inflicted: EngineTime,         // 0x818
    pub crit_health_rumble_played: bool,        // 0x820
    pub crit_health_rumble_duration_id: DelayId, // 0x824
    pub last_health_update: f32,                // 0x82C
    pub stamina_listener: Ref<IScriptable>,     // 0x830
    pub memory_listener: Ref<IScriptable>,      // 0x840
    pub security_area_type_e3hack: ESecurityAreaType, // 0x850
    pub overlapped_security_zones: RedArray<PersistentId>, // 0x858
    pub interesting_facts: InterestingFacts,    // 0x868
    pub interesting_facts_listeners_ids: InterestingFactsListenersIds, // 0x870
    pub interesting_facts_listeners_functions: InterestingFactsListenersFunctions, // 0x878
    pub vision_mode_controller: Ref<IScriptable>, // 0x880
    pub combat_controller: Ref<IScriptable>,    // 0x890
    pub handling_modifiers: Ref<IScriptable>,   // 0x8A0
    pub cached_gameplay_restrictions: RedArray<TweakDbId>, // 0x8B0
    pub delay_end_grace_period_after_spawn_event_id: DelayId, // 0x8C0
    pub cw_mask_in_vehicle_input_held: bool,    // 0x8C8
    pub friendly_devices_hostile_to_enemies_lock: RwLock, // 0x8D0
    pub friendly_devices_hostile_to_enemies: RedArray<EntityId>, // 0x8E0
    pub pocket_radio: Ref<PocketRadio>,         // 0x8F0
    pub boss_that_targets_player: EntityId,     // 0x900
    pub choice_token_text_layer_id: u32,        // 0x908
    pub choice_token_text_drawn: bool,          // 0x90C
}

unsafe impl ScriptClass for PlayerPuppet {
    const NAME: &'static str = "PlayerPuppet";
    type Kind = Scripted;
}

impl AsRef<IScriptable> for PlayerPuppet {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

impl AsRef<Entity> for PlayerPuppet {
    fn as_ref(&self) -> &Entity {
        self.base.as_ref()
    }
}

#[repr(C)]
pub struct ScriptedPuppet {
    pub base: Entity,
    pub _padding0: [u8; PADDING],
    pub ai_controller: Ref<IScriptable>,              // 0xF0
    pub move_policies: Ref<MovePoliciesComponent>,    // 0x100
    pub ai_state_handler_component: Ref<IScriptable>, // 0x110
    pub hit_reaction_component: Ref<IScriptable>,     // 0x120
    pub signal_handler_component: Ref<IScriptable>,   // 0x130
    pub reaction_component: Ref<IScriptable>,         // 0x140
    pub dismemberment_component: Ref<IScriptable>,    // 0x150
    pub hit_represantation: Ref<IScriptable>,         // 0x160
    pub interaction_component: Ref<IScriptable>,      // 0x170
    pub slot_component: Ref<IScriptable>,             // 0x180
    pub senses_component: Ref<IScriptable>,           // 0x190
    pub visible_object_component: Ref<IScriptable>,   // 0x1A0
    pub visible_object_position_updated: bool,        // 0x1B0
    pub sensor_object_component: Ref<IScriptable>,    // 0x1B8
    pub target_tracker_component: Ref<IScriptable>,   // 0x1C8
    pub targeting_components_array: RedArray<Ref<IScriptable>>, // 0x1D8
    pub states_component: Ref<IScriptable>,           // 0x1E8
    pub fx_resource_mapper: Ref<IScriptable>,         // 0x1F8
    pub linked_status_effect: LinkedStatusEffect,     // 0x208
    pub resource_library_component: Ref<IScriptable>, // 0x230
    pub crowd_member_component: Ref<IScriptable>,     // 0x240
    pub inventory_component: WeakRef<IScriptable>,    // 0x250
    pub object_selection_component: Ref<IScriptable>, // 0x260
    pub transform_history_component: Ref<IScriptable>, // 0x270
    pub animation_controller_component: Ref<IScriptable>, // 0x280
    pub bump_component: Ref<IScriptable>,             // 0x290
    pub is_crowd: bool,                               // 0x2A0
    pub incapacitated_on_attach: bool,                // 0x2A1
    pub is_iconic: bool,                              // 0x2A2
    pub combat_hud_manager: Ref<IScriptable>,         // 0x2A8
    pub expose_position: bool,                        // 0x2B8
    pub puppet_state_blackboard: Ref<IScriptable>,    // 0x2C0
    pub custom_blackboard: Ref<IScriptable>,          // 0x2D0
    pub security_area_callback_id: u32,               // 0x2E0
    pub custom_ai_components: RedArray<Ref<IScriptable>>, // 0x2E8
    pub listeners: RedArray<Ref<IScriptable>>,        // 0x2F8
    pub security_support_listener: Ref<IScriptable>,  // 0x308
    pub should_be_revealed_storage: Ref<IScriptable>, // 0x318
    pub input_processed: bool,                        // 0x328
    pub should_spawn_blood_puddle: bool,              // 0x329
    pub blood_puddle_spawned: bool,                   // 0x32A
    pub skip_death_animation: bool,                   // 0x32B
    pub hit_history: Ref<IScriptable>,                // 0x330
    pub current_workspot_tags: RedArray<CName>,       // 0x340
    pub loot_quality: GamedataQuality,                // 0x350
    pub has_quest_items: bool,                        // 0x354
    pub active_quality_range_interaction: CName,      // 0x358
    pub dropped_weapons: bool,                        // 0x360
    pub weakspot_component: Ref<IScriptable>,         // 0x368
    pub breach_controller_component: Ref<IScriptable>, // 0x378
    pub highlight_data: Ref<IScriptable>,             // 0x388
    pub current_tags_stack: u32,                      // 0x398
    pub killer: WeakRef<Entity>,                      // 0x3A0
    pub object_actions_callback_ctrl: Ref<IScriptable>, // 0x3B0
    pub is_active_cached: CachedBoolValue,            // 0x3C0
    pub is_cyberpsycho: bool,                         // 0x3C3
    pub is_civilian: bool,                            // 0x3C4
    pub is_police: bool,                              // 0x3C5
    pub is_ganger: bool,                              // 0x3C6
    pub currently_uploading_action: WeakRef<IScriptable>, // 0x3C8
    pub gameplay_role_component: WeakRef<IScriptable>, // 0x3D8
    pub active_quickhack_action_history: RedArray<Ref<IScriptable>>, // 0x3E8
    pub completed_quickhack_history: RedArray<Ref<IScriptable>>, // 0x3F8
    pub is_finsher_sound_played: bool,                // 0x408
    pub attempted_shards: RedArray<ItemId>,           // 0x410
}

unsafe impl ScriptClass for ScriptedPuppet {
    const NAME: &'static str = "ScriptedPuppet";
    type Kind = Scripted;
}

impl AsRef<IScriptable> for ScriptedPuppet {
    fn as_ref(&self) -> &IScriptable {
        self.base.as_ref()
    }
}

impl AsRef<Entity> for ScriptedPuppet {
    fn as_ref(&self) -> &Entity {
        &self.base
    }
}

pub trait AsScriptedPuppet {
    fn get_npc_type(&self) -> GamedataNpcType;
}

impl AsScriptedPuppet for Ref<ScriptedPuppet> {
    fn get_npc_type(&self) -> GamedataNpcType {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(ScriptedPuppet::NAME)).unwrap();
        let method = cls.get_method(CName::new("GetNPCType;")).ok().unwrap();
        method
            .as_function()
            .execute::<_, GamedataNpcType>(unsafe { self.instance() }.map(AsRef::as_ref), ())
            .unwrap()
    }
}

pub trait AsScriptedPuppetExt {
    /// method found in .reds
    fn get_template_gender(&self) -> audioware_manifest::PlayerGender;
}

impl AsScriptedPuppetExt for Ref<ScriptedPuppet> {
    fn get_template_gender(&self) -> audioware_manifest::PlayerGender {
        let rtti = RttiSystem::get();
        let cls = rtti.get_class(CName::new(ScriptedPuppet::NAME)).unwrap();
        let method = cls.get_method(CName::new("TemplateGender;")).ok().unwrap();
        method
            .as_function()
            .execute::<_, audioware_manifest::PlayerGender>(
                unsafe { self.instance() }.map(AsRef::as_ref),
                (),
            )
            .unwrap()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamedataNpcType {
    Android = 0,
    #[default]
    Any = 1,
    Cerberus = 2,
    Chimera = 3,
    Device = 4,
    Drone = 5,
    Human = 6,
    Mech = 7,
    Spiderbot = 8,
    Count = 9,
    Invalid = 10,
}

unsafe impl NativeRepr for GamedataNpcType {
    const NAME: &'static str = "gamedataNPCType";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GameplayTier {
    Undefined = 0,
    Tier1FullGameplay = 1,
    Tier2StagedGameplay = 2,
    Tier3LimitedGameplay = 3,
    Tier4FppCinematic = 4,
    Tier5Cinematic = 5,
}

unsafe impl NativeRepr for GameplayTier {
    const NAME: &'static str = "GameplayTier";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum GamePsmVehicle {
    Any = -1,
    Default = 0,
    Driving = 1,
    Combat = 2,
    Passenger = 3,
    Transition = 4,
    Turret = 5,
    DriverCombat = 6,
    Scene = 7,
}

unsafe impl NativeRepr for GamePsmVehicle {
    const NAME: &'static str = "gamePSMVehicle";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamedataQuality {
    Common = 0,
    CommonPlus = 1,
    Epic = 2,
    EpicPlus = 3,
    Iconic = 4,
    Legendary = 5,
    LegendaryPlus = 6,
    LegendaryPlusPlus = 7,
    Random = 8,
    Rare = 9,
    RarePlus = 10,
    Uncommon = 11,
    UncommonPlus = 12,
    Count = 13,
    Invalid = 14,
}

unsafe impl NativeRepr for GamedataQuality {
    const NAME: &'static str = "gamedataQuality";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ESecurityAreaType {
    Disabled = 0,
    Safe = 1,
    Restricted = 2,
    Dangerous = 3,
}

unsafe impl NativeRepr for ESecurityAreaType {
    const NAME: &'static str = "ESecurityAreaType";
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct PersistentId {
    pub entity_hash: u64,      // 0x0
    pub component_name: CName, // 0x8
}

unsafe impl NativeRepr for PersistentId {
    const NAME: &'static str = "gamePersistentID";
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct NewMappinId {
    pub value: u64, // 0x0
}

unsafe impl NativeRepr for NewMappinId {
    const NAME: &'static str = "gameNewMappinID";
}

#[repr(C)]
pub struct CallbackHandle {
    pub base: IScriptable,
}

unsafe impl ScriptClass for CallbackHandle {
    const NAME: &'static str = "redCallbackObject";
    type Kind = Native;
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct RwLock {
    pub _padding0: [u8; 0x10],
}

unsafe impl NativeRepr for RwLock {
    const NAME: &'static str = "ScriptReentrantRWLock";
}

#[repr(C)]
pub struct PlayerPhone {}

unsafe impl ScriptClass for PlayerPhone {
    const NAME: &'static str = "PlayerPhone";
    type Kind = Scripted;
}

#[repr(C, align(8))]
pub struct LinkedStatusEffect {
    pub _padding0: [u8; 0x28],
}

unsafe impl NativeRepr for LinkedStatusEffect {
    const NAME: &'static str = "LinkedStatusEffect";
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct InterestingFacts {
    pub _padding0: [u8; 0x8],
}

unsafe impl NativeRepr for InterestingFacts {
    const NAME: &'static str = "InterestingFacts";
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct InterestingFactsListenersFunctions {
    pub _padding0: [u8; 0x8],
}

unsafe impl NativeRepr for InterestingFactsListenersFunctions {
    const NAME: &'static str = "InterestingFactsListenersFunctions";
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct InterestingFactsListenersIds {
    pub _padding0: [u8; 0x4],
}

unsafe impl NativeRepr for InterestingFactsListenersIds {
    const NAME: &'static str = "InterestingFactsListenersIds";
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct DelayId {
    pub _padding0: [u8; 0x8],
}

unsafe impl NativeRepr for DelayId {
    const NAME: &'static str = "gameDelayID";
}

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct CachedBoolValue {
    pub _padding0: [u8; 0x3],
}

unsafe impl NativeRepr for CachedBoolValue {
    const NAME: &'static str = "AIUtilsCachedBoolValue";
}

#[repr(C)]
pub struct MovePoliciesComponent {
    pub base: IScriptable,
    pub name: CName, // 0x40
    pub _padding0: [u8; 0x18],
    pub id: Cruid, // 0x60
    pub _padding1: [u8; 0x23],
    pub is_enabled: bool,    // 0x8B
    pub is_replicable: bool, // 0x8C
    pub _padding2: [u8; 0x553],
}

unsafe impl ScriptClass for MovePoliciesComponent {
    const NAME: &'static str = "movePoliciesComponent";
    type Kind = Native;
}

impl AsRef<IScriptable> for MovePoliciesComponent {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        &self.base
    }
}

#[repr(C)]
pub struct PocketRadio {
    pub player: WeakRef<PlayerPuppet>,    // 0x0
    pub station: i32,                     // 0x10
    pub selected_station: i32,            // 0x14
    pub toggled_station: i32,             // 0x18
    pub restrictions: RedArray<bool>,     // 0x20
    pub is_condition_restricted: bool,    // 0x30
    pub is_unlock_delay_restricted: bool, // 0x31
    pub is_restriction_overwritten: bool, // 0x32
    pub is_on: bool,                      // 0x33
    pub quest_content_lock_listener: Ref<PocketRadioQuestContentLockListener>, // 0x38
    pub radio_press_time: f32,            // 0x48
    pub is_in_metro: bool,                // 0x4C
    pub settings: Ref<RadioportSettingsListener>, // 0x50
}

unsafe impl ScriptClass for PocketRadio {
    const NAME: &'static str = "PocketRadio";
    type Kind = Scripted;
}

#[repr(C)]
pub struct RadioportSettingsListener {
    pub player: WeakRef<PlayerPuppet>,     // 0x0
    pub settings: Ref<IScriptable>,        // 0x10
    pub settings_group: Ref<IScriptable>,  // 0x20
    pub sync_to_car_radio: bool,           // 0x30
    pub cycle_on_button_press: bool,       // 0x31
    pub save_station: bool,                // 0x32
    pub sync_to_car_radio_name: CName,     // 0x38
    pub cycle_on_button_press_name: CName, // 0x40
    pub save_station_name: CName,          // 0x48
    pub radioport_settings_path: CName,    // 0x50
}

unsafe impl ScriptClass for RadioportSettingsListener {
    const NAME: &'static str = "RadioportSettingsListener";
    type Kind = Scripted;
}

#[repr(C)]
pub struct PocketRadioQuestContentLockListener {
    pub pocket_radio: Ref<PocketRadio>, // 0x0
}

unsafe impl ScriptClass for PocketRadioQuestContentLockListener {
    const NAME: &'static str = "PocketRadioQuestContentLockListener";
    type Kind = Scripted;
}
