// Game.TestAudioEvent("dry_fire");
public static exec func TestAudioEvent(game: GameInstance, name: String) -> Void {
    let player = GetPlayer(game);
    let sound: CName = StringToName(name);
    let event: ref<AudioEvent> = new AudioEvent();
    event.eventName = sound;
    player.QueueEvent(event);
}