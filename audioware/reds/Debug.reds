import Audioware.Audioware
import Audioware.AudiowareSettingsDef

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
    let sound: CName = StringToName(name);
    GameObject.PlaySoundEvent(player, sound);
}

// Game.TestStopSoundEvent("ono_v_effort_short");
public static exec func TestStopSoundEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameObject.StopSoundEvent(player, sound);
}

// Game.TestEntityQueueEvent("ono_v_effort_long");
public static exec func TestEntityQueueEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let evt: ref<SoundPlayEvent> = new SoundPlayEvent();
    evt.soundName = StringToName(name);
    player.QueueEvent(evt);
}

// Game.TestAudioSystemPlay("ono_v_effort_short");
// Game.TestAudioSystemPlay("nah_everything_is_all_good");
public static exec func TestAudioSystemPlay(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameInstance.GetAudioSystem(game).Play(sound, player.GetEntityID(), n"V");
}

// Game.TestAudioSystemStop("ono_v_effort_short");
public static exec func TestAudioSystemStop(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    GameInstance.GetAudioSystem(game).Stop(sound, player.GetEntityID(), n"V");
}

// Game.TestUpdateReverb(0.); // between 0. and 1.
public static exec func TestUpdateReverb(gi: GameInstance, value: Float) -> Void {
    let defs = GetAllBlackboardDefs();
    let boards = GameInstance.GetBlackboardSystem(gi);
    let board = boards.Get(defs.AudiowareSettings);
    board.SetFloat(defs.AudiowareSettings.PlayerReverb, value, true);
}

// Game.TestUpdatePlayerPreset(0); // 0, 1 or 2
public static exec func TestUpdatePlayerPreset(gi: GameInstance, value: Int32) -> Void {
    let defs = GetAllBlackboardDefs();
    let boards = GameInstance.GetBlackboardSystem(gi);
    let board = boards.Get(defs.AudiowareSettings);
    board.SetInt(defs.AudiowareSettings.PlayerPreset, value, true);
}

// Game.TestPlayOverThePhone("nah_everything_is_all_good", "V");
public static exec func TestPlayOverThePhone(gi: GameInstance, name: String, emitter: String) -> Void {
    let sound: CName = StringToName(name);
    let speaker: CName = StringToName(emitter);
    GameInstance.GetAudioSystem(gi).PlayOverThePhone(sound, speaker);
}

// Game.ApplyVentriloquistOnNPC();
public static exec func ApplyVentriloquistOnNPC(gi: GameInstance) -> Void {
  let player: ref<PlayerPuppet> = GetPlayer(gi);
  let id = GameInstance.GetTargetingSystem(gi).GetLookAtObject(player).GetEntityID();
  let entity = GameInstance.FindEntityByID(gi, id);
  
  if IsDefined(entity) {
    let system = Audioware.GetInstance(player.GetGame());
    system.RegisterVentriloquist(id);
    let callback = new RepeatSameSoundCallback();
    callback.npc = entity as GameObject;
    GameInstance.GetDelaySystem(gi).DelayCallback(callback, 3.0, true);
  }
}

public class RepeatSameSoundCallback extends DelayCallback {
  public let npc: wref<GameObject>;
  public func Call() -> Void {
    if IsDefined(this.npc) {
      let id = this.npc.GetEntityID();
      GameInstance
      .GetAudioSystem(this.npc.GetGame())
      .Play(n"nah_everything_is_all_good", id, n"Jean-Claude");
      GameInstance.GetDelaySystem(this.npc.GetGame()).DelayCallback(this, 5.0, true);
    }
  }
}