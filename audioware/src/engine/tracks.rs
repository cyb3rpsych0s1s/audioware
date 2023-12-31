use std::sync::OnceLock;

use kira::{
    manager::AudioManager,
    track::{effect::reverb::ReverbBuilder, TrackBuilder, TrackHandle, TrackRoutes},
};
use lazy_static::lazy_static;

lazy_static! {
    static ref TRACKS: OnceLock<Tracks> = OnceLock::default();
}

#[allow(dead_code)]
struct Tracks {
    reverb: TrackHandle,
    v: V,
}

#[allow(dead_code)]
struct V {
    main: TrackHandle,
    vocal: TrackHandle,
    mental: TrackHandle,
    emissive: TrackHandle,
}

pub fn setup(manager: &mut AudioManager) -> anyhow::Result<()> {
    let reverb = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(ReverbBuilder::new().mix(1.0));
        builder
    })?;
    let main = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&reverb, 0.25)))?;
    let vocal = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    let mental = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    let emissive = manager
        .add_sub_track(TrackBuilder::new().routes(TrackRoutes::new().with_route(&main, 1.)))?;
    if TRACKS
        .set(Tracks {
            reverb,
            v: V {
                main,
                vocal,
                mental,
                emissive,
            },
        })
        .is_err()
    {
        red4ext_rs::error!("error initializing tracks for audio engine");
    }
    Ok(())
}

pub fn vocal<'a>() -> Option<&'a TrackHandle> {
    TRACKS.get().map(|x| &x.v.vocal)
}
