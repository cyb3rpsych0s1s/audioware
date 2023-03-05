module Audioware.System

import Audioware.Plugin

public class AudiowareSystem extends ScriptableSystem {
  private let plugin: wref<Plugin>;

  private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
      this.plugin = Plugin.Initialize();
  }
  
  public final static func GetInstance(gameInstance: GameInstance) -> ref<AudiowareSystem> {
    let container = GameInstance.GetScriptableSystemsContainer(gameInstance);
    return container.Get(n"Audioware.System.AudiowareSystem") as AudiowareSystem;
  }
}
