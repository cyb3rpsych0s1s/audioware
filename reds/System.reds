module Audioware.System

import Audioware.Plugin

public class AudiowareSystem extends ScriptableSystem {
  private let plugin: wref<Plugin>;

  private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
      this.plugin = Plugin.Initialize();
  }
}
