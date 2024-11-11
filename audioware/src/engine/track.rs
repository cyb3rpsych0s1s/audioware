use kira::track::TrackHandle;

pub struct Tracks {
    // tracks affected by reverb mix + preset (e.g. underwater)
    pub ambience: TrackHandle,
}
