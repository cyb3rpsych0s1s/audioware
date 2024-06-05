import Audioware.Audioware
import Audioware.LinearTween
import Audioware.ElasticTween
import Audioware.Easing

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

public static exec func TestAudioSystemExtStop(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    let tween: ElasticTween = new ElasticTween(0u, 300u, Easing.InPowi);
    GameInstance.GetAudioSystem(game).Stop(sound, player.GetEntityID(), n"V", tween);
}
