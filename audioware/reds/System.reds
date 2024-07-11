module Audioware

public class AudiowareSystem extends ScriptableSystem {
    private let plugin: ref<AudiowarePlugin>;
    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
        this.plugin = new AudiowarePlugin();
        this.plugin.Yolo();
    }
    private func OnDetach() -> Void {
        this.plugin = null;
    }
}