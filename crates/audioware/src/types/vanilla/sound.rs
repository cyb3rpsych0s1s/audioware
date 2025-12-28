use std::ops::Not;

use bitflags::bitflags;
use red4ext_rs::types::{CName, EntityId, RedArray, ResRef};

use crate::{AudioEventId, RedTagList, SoundEngine, SoundObjectId, Vector4, WwiseId};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Pair<T> {
    pub name: CName, // 0
    pub value: T,    // 8
}

impl<T> Copy for Pair<T> where T: Copy {}

impl<T> std::fmt::Display for Pair<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name.as_str(), self.value)
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TFlag: u8 {
        const PENDING               = 1 << 0;
        const WAITING_FOR_ACOUSTICS = 1 << 1;
        const HAS_RAYCAST_OCCLUSION = 1 << 2;
        const HAS_GRAPH_OCCLUSION   = 1 << 3;
        const IS_IN_DIFFERENT_ROOM  = 1 << 4;
    }
}

impl std::fmt::Display for TFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = vec![];
        if self.contains(Self::PENDING) {
            out.push("P");
        }
        if self.contains(Self::WAITING_FOR_ACOUSTICS) {
            out.push("WA");
        }
        if self.contains(Self::HAS_RAYCAST_OCCLUSION) {
            out.push("HRO");
        }
        if self.contains(Self::HAS_GRAPH_OCCLUSION) {
            out.push("HGO");
        }
        if self.contains(Self::IS_IN_DIFFERENT_ROOM) {
            out.push("IDR");
        }
        if out.is_empty() {
            write!(f, "none")
        } else {
            write!(f, "({})", out.join("|"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SoundId(u32);

impl std::fmt::Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sid:{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PlayingSoundId(AudioEventId);

impl PlayingSoundId {
    pub fn invalid() -> Self {
        Self(AudioEventId::invalid())
    }
}

impl std::fmt::Display for PlayingSoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "psid:{}", self.0)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct OneShotSound {
    position: Vector4,               // 0
    params: RedArray<Pair<f32>>,     // 10
    switches: RedArray<Pair<CName>>, // 20
    event_name: CName,               // 30
    graph_occlusion: f32,            // 38
    raycast_occlusion: f32,          // 3C
    event_id: WwiseId,               // 40
    flags: TFlag,                    // 44
}

impl OneShotSound {
    pub fn event_name(&self) -> CName {
        self.event_name
    }
    /// # Safety
    /// can only be called once SoundEngine has been initialized.
    pub unsafe fn wwise_id(&self) -> WwiseId {
        unsafe { SoundEngine::get() }
            .metadata_manager()
            .event_wwise_id(self.event_name)
    }
    pub fn params(&self) -> &[Pair<f32>] {
        self.params.iter().as_slice()
    }
    pub fn switches(&self) -> &[Pair<CName>] {
        self.switches.iter().as_slice()
    }
}

impl std::fmt::Display for OneShotSound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self {
            position,
            ref params,
            ref switches,
            event_name,
            graph_occlusion,
            raycast_occlusion,
            event_id,
            flags,
        } = self;
        write!(
            f,
            "position: {position}, params: [{}], switches: [{}], event_name: {event_name}, graph_occlusion: {graph_occlusion}, raycast_occlusion: {raycast_occlusion}, event_id: {event_id}, flags: {flags}",
            params
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            switches
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

const PADDING_SO_30: usize = 0x34 - 0x30;

#[derive(Debug)]
#[repr(C)]
pub struct SoundObject {
    sound_tags: *const RedArray<CName>, // 0
    emitter_tags: *const RedTagList,    // 8
    id: SoundObjectId,                  // 10
    position_entry: *mut PositionEntry, // 18
    emitter_name: CName,                // 20
    entity_id: EntityId,                // 28
    unk30: [u8; PADDING_SO_30],         // 30
    is_in_listener_room: bool,          // 34
    force_property_update: bool,        // 35
    is_one_shot: bool,                  // 36
}

impl SoundObject {
    pub fn id(&self) -> SoundObjectId {
        self.id
    }
    pub fn emitter_name(&self) -> CName {
        self.emitter_name
    }
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }
    pub fn sound_tags(&self) -> Option<&RedArray<CName>> {
        if !self.sound_tags.is_null() {
            return Some(unsafe { &*self.sound_tags });
        }
        None
    }
    pub fn emitter_tags(&self) -> Option<&RedArray<CName>> {
        if !self.emitter_tags.is_null() {
            return Some(&unsafe { &*self.emitter_tags }.tags);
        }
        None
    }
}

impl std::fmt::Display for SoundObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            id,
            emitter_name,
            entity_id,
            is_in_listener_room,
            force_property_update,
            is_one_shot,
            ..
        } = self;
        write!(
            f,
            "{id}, {entity_id}, emitter_name: {emitter_name}, sound_tags: [{}], emitter_tags: [{}], position: {}, is_in_listener_room: {is_in_listener_room}, force_property_update: {force_property_update}, is_one_shot: {is_one_shot}",
            if self.sound_tags.is_null().not() {
                unsafe { &*self.sound_tags }
                    .iter()
                    .copied()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                Default::default()
            },
            if self.emitter_tags.is_null().not() {
                unsafe { &*self.emitter_tags }
                    .tags
                    .iter()
                    .copied()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                Default::default()
            },
            if self.position_entry.is_null().not() {
                format!("{{ {} }}", unsafe { &*self.position_entry })
            } else {
                "undefined".to_string()
            }
        )
    }
}

const PADDING_PE_30: usize = 0x50 - 0x30;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PositionEntry {
    position: Vector4,            // 0
    out_position: Vector4,        // 10
    entity_id: EntityId,          // 20
    distance_fade_int: u16,       // 28
    input_distance_fade_int: u16, // 2A
    name_index: u16,              // 2C
    panning_key: u8,              // 2E
    flags: PEFlag,                // 2F
    unk30: [u8; PADDING_PE_30],   // 30
}

impl std::fmt::Display for PositionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: {}, out_position: {}, entity_id: {}, distance_fade_int: {}, input_distance_fade_int: {}, name_index: {}, panning_key: {}, flags: {}",
            self.position,
            self.out_position,
            self.entity_id,
            self.distance_fade_int,
            self.input_distance_fade_int,
            self.name_index,
            if self.panning_key != 255 {
                self.panning_key.to_string()
            } else {
                "invalid".to_string()
            },
            self.flags,
        )
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PEFlag: u8 {
        const ACOUSTIC_REPOSITIONING  = 1 << 0;
        const FIRST_ACQUIRE_DONE      = 1 << 2;
        const TARGET_REPOSITIONING_VO = 1 << 3;
    }
}

impl std::fmt::Display for PEFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = vec![];
        if self.contains(PEFlag::ACOUSTIC_REPOSITIONING) {
            out.push("AR");
        }
        if self.contains(PEFlag::FIRST_ACQUIRE_DONE) {
            out.push("FA");
        }
        if self.contains(PEFlag::TARGET_REPOSITIONING_VO) {
            out.push("TR");
        }
        write!(f, "{}", out.join("|"))
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SoundPlayContext {
    external_filename: ResRef,  // 0
    sound_name: CName,          // 8
    seek: f32,                  // 10
    play_type: PlayContextType, // 14
}

impl std::fmt::Display for SoundPlayContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "external_filename: {}, sound_name: {}, seek: {}, play_type: {}, ..",
            unsafe { std::mem::transmute::<ResRef, u64>(self.external_filename.clone()) },
            self.sound_name.as_str(),
            self.seek,
            self.play_type
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PlayContextType {
    Null,
    Play,
    PlayExternal,
}

impl std::fmt::Display for PlayContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayContextType::Null => "null",
                PlayContextType::Play => "play",
                PlayContextType::PlayExternal => "play external",
            }
        )
    }
}

const PADDING_15: usize = 0x30 - 0x19;
const PADDING_38: usize = 0x50 - 0x38;
const PADDING_54: usize = 0x59 - 0x54;

const PADDING_PAST_PLAY_CONTEXT: usize = 0x30 - 0x18;
const PADDING_PAST_IDS: usize = 0x54 - 0x4C;

#[derive(Debug)]
#[repr(C)]
pub struct Sound {
    play_context: SoundPlayContext,              // 0
    unk: [u8; PADDING_PAST_PLAY_CONTEXT],        // ?
    sound_object: *mut SoundObject,              // 30
    event_metadata: *const std::ffi::c_void,     // 38
    sound_id: SoundId,                           // 40
    playing_id: PlayingSoundId,                  // 44
    event_id: AudioEventId,                      // 48
    unkx: [u8; PADDING_PAST_IDS],                // ?
    duration: f32,                               // 54
    callback_flags: u8,                          // 58
    state: State,                                // 59
    grouping_weight: u8,                         // ?
    stop_requested: bool,                        // 5B
    virtualization_waiting_for_sound_stop: bool, // ?
    has_ended: bool,                             // 5D
    is_using_its_sound_object: bool,             // 5E
}

impl Sound {
    pub fn sound_object(&self) -> Option<&SoundObject> {
        if self.sound_object.is_null().not() {
            return Some(unsafe { &*self.sound_object });
        }
        None
    }
    pub fn sound_name(&self) -> CName {
        self.play_context.sound_name
    }
    pub fn external_resource_path(&self) -> ResRef {
        self.play_context.external_filename.clone()
    }
    /// # Safety
    /// can only be called once SoundEngine has been initialized.
    pub unsafe fn wwise_id(&self) -> WwiseId {
        unsafe { SoundEngine::get() }
            .metadata_manager()
            .event_wwise_id(self.play_context.sound_name)
    }
    pub fn is_play_external(&self) -> bool {
        self.play_context.play_type == PlayContextType::PlayExternal
    }
}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            play_context,
            sound_id,
            playing_id,
            event_id,
            duration,
            stop_requested,
            virtualization_waiting_for_sound_stop,
            has_ended,
            is_using_its_sound_object,
            sound_object,
            ..
        } = self;
        write!(
            f,
            "play_context: {{ {play_context} }}, {sound_id}, {playing_id}, {event_id}, duration: {duration}, stop_requested: {stop_requested}, virtualization_waiting_for_sound_stop: {virtualization_waiting_for_sound_stop}, has_ended: {has_ended}, is_using_its_sound_object: {is_using_its_sound_object}, {}",
            if sound_object.is_null().not() {
                format!("sound_object: {{ {} }}", unsafe { &**sound_object })
            } else {
                Default::default()
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum State {
    Initializing,
    ToPlay,
    Devirtualizing,
    Playing,
    Virtualizing,
    Virtual,
    Stopping,
    Stopped,
    Free,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Initializing => "initializing",
                State::ToPlay => "to_play",
                State::Devirtualizing => "devirtualizing",
                State::Playing => "playing",
                State::Virtualizing => "virtualizing",
                State::Virtual => "virtual",
                State::Stopping => "stopping",
                State::Stopped => "stopped",
                State::Free => "free",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout() {
        assert_eq!(std::mem::size_of::<OneShotSound>(), 80);
    }
}
