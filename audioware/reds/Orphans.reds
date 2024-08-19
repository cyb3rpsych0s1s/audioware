import Audioware.AudioSystemExt

@addMethod(GameInstance)
public static func GetAudioSystemExt(game: GameInstance) -> ref<AudioSystemExt> {
    return GameInstance
    .GetScriptableSystemsContainer(game)
    .Get(n"Audioware.AudioSystemExt") as AudioSystemExt;
}