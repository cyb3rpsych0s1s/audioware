use audioware_manifest::ScnDialogLineType;
use red4ext_rs::{
    RttiSystem,
    types::{CName, EntityId},
};

/// Trigger subtitles display in Redscript from Rust.
pub fn propagate_subtitles(
    reaction: CName,
    entity_id: EntityId,
    emitter_name: CName,
    line_type: ScnDialogLineType,
    duration: f32,
) {
    let rtti = RttiSystem::get();
    let methods = rtti.get_global_functions();
    let method = methods
        .iter()
        .find(|x| {
            x.name()
                == CName::new(
                    "Audioware.PropagateSubtitle;CNameEntityIDCNamescnDialogLineTypeFloat",
                )
        })
        .unwrap();
    method
        .execute::<_, ()>(
            None,
            (reaction, entity_id, emitter_name, line_type, duration),
        )
        .unwrap();
}
