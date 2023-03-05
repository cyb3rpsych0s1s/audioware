module Audioware.System

import Audioware.Plugin
import Audioware.Utils.E

public class AudiowareSystem extends ScriptableSystem {
  private let plugin: wref<Plugin>;

  private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
      E(s"on player attach");
      this.plugin = Plugin.Initialize();
  }
  
  public final static func GetInstance(gameInstance: GameInstance) -> ref<AudiowareSystem> {
    let container = GameInstance.GetScriptableSystemsContainer(gameInstance);
    return container.Get(n"Audioware.System.AudiowareSystem") as AudiowareSystem;
  }

  public func PlayCustomSound(sound: String) -> Void {
    E(s"play custom sound");
    Plugin.Load(this.plugin, sound);
  }
}
