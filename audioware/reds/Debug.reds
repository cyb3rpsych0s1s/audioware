import Audioware.AudiowareSystem
import Audioware.CallYoloOn

/// Game.TestSyxtem();
public static exec func TestSyxtem(game: GameInstance) {
    let container = GameInstance.GetScriptableSystemsContainer(game);
    let system = container.Get(n"Audioware.AudiowareSystem") as AudiowareSystem;
    CallYoloOn(system);
}