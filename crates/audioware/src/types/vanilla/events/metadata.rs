use red4ext_rs::{
    types::{CName, ISerializable, RedArray},
    NativeRepr,
};

use crate::types::Vector3;

use super::ESoundCurveType;

#[derive(Debug)]
#[repr(C)]
pub struct AudioMetadataBase {
    base: ISerializable, // 0
    pub name: CName,     // 30
}

unsafe impl NativeRepr for AudioMetadataBase {
    const NAME: &'static str = "audioAudioMetadataBase";
}

#[derive(Debug)]
#[repr(C)]
pub struct AudioMetadata {
    base: AudioMetadataBase,
}

unsafe impl NativeRepr for AudioMetadata {
    const NAME: &'static str = "audioAudioMetadata";
}

#[derive(Debug)]
#[repr(C)]
pub struct CustomEmitterMetadata {
    base: AudioMetadata,
}

unsafe impl NativeRepr for CustomEmitterMetadata {
    const NAME: &'static str = "audioCustomEmitterMetadata";
}

const PADDING_6F: usize = 0x70 - 0x6F;
const PADDING_74: usize = 0x78 - 0x74;
const PADDING_9C: usize = 0xA0 - 0x9C;
const PADDING_384: usize = 0x388 - 0x384;
const PADDING_398: usize = 0x3A8 - 0x398;

#[derive(Debug)]
#[repr(C)]
pub struct VehicleMetadata {
    base: CustomEmitterMetadata,                           // 0
    pub vehicle_collision_settings: CName,                 // 38
    pub vehicle_grid_destruction_settings: CName,          // 40
    pub vehicle_part_settings: CName,                      // 48
    pub collision_cooldown: f32,                           // 50
    pub min_rpm: f32,                                      // 54
    pub max_rpm: f32,                                      // 58
    pub max_playing_distance: f32,                         // 5C
    pub doppler_factor: f32,                               // 60
    pub suspension_squeek_timeout: f32,                    // 64
    pub exit_delay: f32,                                   // 68
    pub test_wheel_material: bool,                         // 6C
    pub has_radio_receiver: bool,                          // 6D
    pub uses_police_radio_station: bool,                   // 6E
    unk6f: [u8; PADDING_6F],                               // 6F
    pub acoustic_isolation_factor: f32,                    // 70
    unk74: [u8; PADDING_74],                               // 74
    pub traffic_emitter_metadata: CName,                   // 78
    pub radio_receiver_type: CName,                        // 80
    pub matching_startup_radio_stations: RedArray<CName>,  // 88
    pub radio_plays_when_engine_starts_probability: f32,   // 98
    pub unk9c: [u8; PADDING_9C],                           // 9C
    pub general_data: VehicleGeneralData,                  // A0
    pub mechanical_data: VehicleMechanicalData,            // 228
    pub wheel_data: VehicleWheelData,                      // 2C0
    pub emitter_position_data: VehicleEmitterPositionData, // 318
    unk384: [u8; PADDING_384],                             // 384
    pub gear_sweeteners: RedArray<CName>,                  // 388
    unk398: [u8; PADDING_398],                             // 398
}

unsafe impl NativeRepr for VehicleMetadata {
    const NAME: &'static str = "audioVehicleMetadata";
}

const PADDING_184: usize = 0x188 - 0x184;

#[derive(Debug)]
#[repr(C)]
pub struct VehicleGeneralData {
    pub rev_soundbank_name: CName,                                     // 00
    pub rev_electric_soundbank_name: CName,                            // 08
    pub reverb_soundbank_name: CName,                                  // 10
    pub enter_vehicle_event: CName,                                    // 18
    pub exit_vehicle_event: CName,                                     // 20
    pub ignition_start_event: CName,                                   // 28
    pub ignition_end_event: CName,                                     // 30
    pub ui_start_event: CName,                                         // 38
    pub ui_end_event: CName,                                           // 40
    pub horn_on_event: CName,                                          // 48
    pub horn_off_event: CName,                                         // 50
    pub police_horn_on_event: CName,                                   // 58
    pub police_horn_off_event: CName,                                  // 60
    pub traffic_panic_horn_on_event: CName,                            // 68
    pub traffic_panic_horn_off_event: CName,                           // 70
    pub siren_on_event: CName,                                         // 78
    pub siren_off_event: CName,                                        // 80
    pub rain_start_event: CName,                                       // 88
    pub rain_stop_event: CName,                                        // 90
    pub water_start_event: CName,                                      // 98
    pub water_stop_event: CName,                                       // A0
    pub tyre_burst_event: CName,                                       // A8
    pub collision_sound_event: CName,                                  // B0
    pub brake_apply_event: CName,                                      // B8
    pub brake_release_event: CName,                                    // C0
    pub handbrake_apply_event: CName,                                  // C8
    pub handbrake_release_event: CName,                                // D0
    pub brake_loop_start_event: CName,                                 // D8
    pub brake_loop_end_event: CName,                                   // E0
    pub lights_on_event: CName,                                        // E8
    pub lights_off_event: CName,                                       // F0
    pub interior_reverb_bus: CName,                                    // F8
    pub skid: CName,                                                   // 100
    pub inclination: CName,                                            // 108
    pub impact_velocity: CName,                                        // 110
    pub doppler_shift: CName,                                          // 118
    pub acousting_isolation_factor: CName,                             // 120
    pub impact_grid_cell_raw_change: CName,                            // 128
    pub vehicle_doors_settings: VehicleDoorsSettingsMetadata,          // 130
    pub vehicle_interior_parameter_data: VehicleInteriorParameterData, // 160
    pub vehicle_temperature_settings: VehicleTemperatureSettings,      // 178
    unk184: [u8; PADDING_184],                                         // 184
}

unsafe impl NativeRepr for VehicleGeneralData {
    const NAME: &'static str = "audioVehicleGeneralData";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleDoorsSettingsMetadata {
    pub door: VehicleDoorsSettings,  // 00
    pub trunk: VehicleDoorsSettings, // 10
    pub hood: VehicleDoorsSettings,  // 20
}

unsafe impl NativeRepr for VehicleDoorsSettingsMetadata {
    const NAME: &'static str = "audioVehicleDoorsSettingsMetadata";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleDoorsSettings {
    pub open_event: CName,  // 00
    pub close_event: CName, // 08
}

unsafe impl NativeRepr for VehicleDoorsSettings {
    const NAME: &'static str = "audioVehicleDoorsSettings";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleInteriorParameterData {
    pub enter_curve_type: ESoundCurveType, // 00
    pub enter_curve_time: f32,             // 04
    pub enter_delay_time: f32,             // 08
    pub exit_curve_type: ESoundCurveType,  // 0C
    pub exit_curve_time: f32,              // 10
    pub exit_delay_time: f32,              // 14
}

unsafe impl NativeRepr for VehicleInteriorParameterData {
    const NAME: &'static str = "audioVehicleInteriorParameterData";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleTemperatureSettings {
    pub rpm_threshold: f32,                // 00
    pub time_to_activate_temperature: f32, // 04
    pub cooldown_time: f32,                // 08
}

unsafe impl NativeRepr for VehicleTemperatureSettings {
    const NAME: &'static str = "audioVehicleTemperatureSettings";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleMechanicalData {
    pub engine_start_event: CName,          // 00
    pub engine_stop_event: CName,           // 08
    pub gear_up_begin_event: CName,         // 10
    pub gear_up_end_event: CName,           // 18
    pub gear_down_begin_event: CName,       // 20
    pub gear_down_end_event: CName,         // 28
    pub throttle_on_event: CName,           // 30
    pub throttle_off_event: CName,          // 38
    pub suspension_squeek_event: CName,     // 40
    pub full_throttle_applied_event: CName, // 48
    pub acelleration: CName,                // 50
    pub speed: CName,                       // 58
    pub gear: CName,                        // 60
    pub brake: CName,                       // 68
    pub rpm: CName,                         // 70
    pub throttle: CName,                    // 78
    pub sideways_throttle: CName,           // 80
    pub thrust: CName,                      // 88
    pub temperature: CName,                 // 90
}

unsafe impl NativeRepr for VehicleMechanicalData {
    const NAME: &'static str = "audioVehicleMechanicalData";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleWheelData {
    pub suspension_pressure_multiplier: f32,               // 00
    pub landing_suspension_pressure_multiplier: f32,       // 04
    pub suspension_pressure_limit: f32,                    // 08
    pub min_suspension_pressure_threshold: f32,            // 0C
    pub suspension_impact_cooldown: f32,                   // 10
    pub min_wheel_time_in_air_before_landing: f32,         // 14
    pub wheel_start_events: RedArray<CName>,               // 18
    pub wheel_stop_events: RedArray<CName>,                // 28
    pub wheel_regular_suspension_impacts: RedArray<CName>, // 38
    pub wheel_landing_suspension_impacts: RedArray<CName>, // 48
}

unsafe impl NativeRepr for VehicleWheelData {
    const NAME: &'static str = "audioVehicleWheelData";
}

#[derive(Debug)]
#[repr(C)]
pub struct VehicleEmitterPositionData {
    pub engine_emitter_position: Vector3,  // 00
    pub central_emitter_position: Vector3, // 0C
    pub exaust_emitter_position: Vector3,  // 18
    pub hood_emitter_position: Vector3,    // 24
    pub trunk_emitter_position: Vector3,   // 30
    pub wheel1_position: Vector3,          // 3C
    pub wheel2_position: Vector3,          // 48
    pub wheel3_position: Vector3,          // 54
    pub wheel4_position: Vector3,          // 60
}

unsafe impl NativeRepr for VehicleEmitterPositionData {
    const NAME: &'static str = "audioVehicleEmitterPositionData";
}
