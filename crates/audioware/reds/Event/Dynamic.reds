module Audioware

enum EventHookType {
    Play = 0,
    PlayOneShot = 1,
    SetParameter = 2,
    StopSound = 3,
    SetSwitch = 4,
    StopTagged = 5,
    PlayExternal = 6,
    Tag = 7,
    Untag = 8,
    SetAppearanceName = 9,
    SetEntityName = 10,
    AddContainerStreamingPrefetch = 11,
    RemoveContainerStreamingPrefetch = 12,
    SetGlobalParameter = 13,
}

public native class DynamicSoundEvent extends Event {
    public final native func SetVolume(value: Float, tween: ref<Tween>);
    public final native func SetPlaybackRate(value: Float, tween: ref<Tween>);
    public final native func SetPanning(value: Float, tween: ref<Tween>);

    public final native func Position() -> Float;

    public final native func Stop(tween: ref<Tween>);
    public final native func Pause(tween: ref<Tween>);
    public final native func Resume(tween: ref<Tween>);
    public final native func ResumeAt(value: Float, tween: ref<Tween>);
    
    public final native func SeekTo(value: Float, tween: ref<Tween>);
    public final native func SeekBy(value: Float, tween: ref<Tween>);
    
    public native static func Create(name: CName, ext: ref<AudioSettingsExt>) -> ref<DynamicSoundEvent>;
    public static func Create(name: CName) -> ref<DynamicSoundEvent> = DynamicSoundEvent.Create(name, null);
}
