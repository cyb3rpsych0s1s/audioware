// Game.TestAudioEvent("dry_fire");
// Game.TestAudioEvent("ono_v_effort_short");
public static exec func TestAudioEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    let event: ref<AudioEvent> = new AudioEvent();
    event.eventName = sound;
    player.QueueEvent(event);
}

// Game.TestPlaySoundEvent("ono_v_effort_short");
public static exec func TestPlaySoundEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    LogChannel(n"DEBUG", s"player is defined: \(ToString(IsDefined(player)))");
    let sound: CName = StringToName(name);
    GameObject.PlaySoundEvent(player, sound);
}

// Game.TestStopSoundEvent("ono_v_effort_short");
public static exec func TestStopSoundEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameObject.StopSoundEvent(player, sound);
}

// Game.TestAudioSystemPlay("ono_v_effort_short");
public static exec func TestAudioSystemPlay(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    // LogChannel(n"DEBUG", s"player.GetEntityID() string: \(EntityID.ToDebugString(player.GetEntityID()))");
    // LogChannel(n"DEBUG", s"player.GetEntityID() string dec: \(EntityID.ToDebugStringDecimal(player.GetEntityID()))");
    // LogChannel(n"DEBUG", s"player.GetEntityID() hash: \(ToString(EntityID.GetHash(player.GetEntityID())))");
    // always `1` for player
    GameInstance.GetAudioSystem(game).Play(sound, player.GetEntityID(), n"V");
}