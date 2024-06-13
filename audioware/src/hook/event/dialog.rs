use super::super::address::{ON_DIALOGLINEEND_EVENT, ON_DIALOGLINE_EVENT, ON_STOPDIALOGLINE_EVENT};
use audioware_core::audioware_dbg;
use audioware_macros::NativeHandler;
use audioware_mem::FromMemory;
use audioware_sys::interop::audio::{
    DialogLineEndEvent, DialogLineEvent, DialogLineEventData, StopDialogLine,
};
use red4ext_rs::conv::ClassType;

pub fn print_event(event: DialogLineEvent) {
    let DialogLineEvent { dialog_line, .. } = event;
    let DialogLineEventData {
        string_id,
        context,
        expression,
        is_player,
        is_rewind,
        is_holocall,
        custom_vo_event,
        seek_time,
        playback_speed_parameter,
        ..
    } = dialog_line;
    audioware_dbg!(
        "intercepted {} ({}): {:?}, {}, {}, {}, {}, {}, {}, {}, {}",
        DialogLineEvent::NAME,
        DialogLineEvent::NATIVE_NAME,
        string_id,
        context,
        expression,
        is_player,
        is_rewind,
        is_holocall,
        custom_vo_event,
        seek_time,
        playback_speed_parameter
    );
}

pub fn print_stop_event(event: StopDialogLine) {
    let StopDialogLine {
        string_id,
        fade_out,
        ..
    } = event;
    audioware_dbg!(
        "intercepted {} ({}): {:?}, {}",
        StopDialogLine::NAME,
        StopDialogLine::NATIVE_NAME,
        string_id,
        fade_out
    );
}

pub fn print_event_end(_: DialogLineEndEvent) {
    audioware_dbg!(
        "intercepted {} ({})",
        DialogLineEndEvent::NAME,
        DialogLineEndEvent::NATIVE_NAME
    );
}

#[derive(NativeHandler)]
#[hook(
    offset = ON_DIALOGLINE_EVENT,
    event = "audioware_sys::interop::audio::DialogLineEvent",
    detour = "print_event"
)]
pub struct HookgameaudioeventsDialogLine;

#[derive(NativeHandler)]
#[hook(
    offset = ON_DIALOGLINEEND_EVENT,
    event = "audioware_sys::interop::audio::DialogLineEndEvent",
    detour = "print_event_end"
)]
pub struct HookgameaudioeventsDialogLineEnd;

#[derive(NativeHandler)]
#[hook(
    offset = ON_STOPDIALOGLINE_EVENT,
    event = "audioware_sys::interop::audio::StopDialogLine",
    detour = "print_stop_event"
)]
pub struct HookgameaudioeventsStopDialogLine;
