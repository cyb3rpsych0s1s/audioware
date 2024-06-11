import Audioware.Audioware

// Game.TestAudioSystemPlay("ono_v_effort_short");
// Game.TestAudioSystemPlay("nah_everything_is_all_good");
// Game.TestAudioSystemPlay("god_love_us");
public static exec func TestAudioSystemPlay(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameInstance.GetAudioSystem(game).Play(sound, player.GetEntityID(), n"V");
}

// Game.TestAudioSystemStop("ono_v_effort_short");
// Game.TestAudioSystemStop("nah_everything_is_all_good");
// Game.TestAudioSystemStop("god_love_us");
public static exec func TestAudioSystemStop(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameInstance.GetAudioSystem(game).Stop(sound, player.GetEntityID(), n"V");
}

// Game.TestAudioSystemExtStop("god_love_us");
public static exec func TestAudioSystemExtStop(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    let tween: ref<AudiowareElasticTween> = new AudiowareElasticTween();
    tween.startTime = 0u;
    tween.duration = 3000u;
    tween.easing = AudiowareEasing.InPowi;
    tween.value = 2;
    GameInstance.GetAudioSystem(game).Stop(sound, player.GetEntityID(), n"V", tween);
}

// Game.TestAudioSystemSwitch("god_love_us", "nah_everything_is_all_good");
public static exec func TestAudioSystemSwitch(game: GameInstance, previous: String, next: String) -> Void {
    let player = GetPlayer(game);
    let previousSound: CName = StringToName(previous);
    let nextSound: CName = StringToName(next);
    GameInstance.GetAudioSystem(game).Switch(previousSound, nextSound, player.GetEntityID(), n"V");
}

// Game.TestModulator(100.0);
public static exec func TestModulator(game: GameInstance, value: Float) -> Void {
    GameInstance.GetAudioSystem(game)
    .GlobalParameter(n"audioware_frequencies", value);
}
