module Audioware

public class AudiowareSystem extends ScriptableSystem {
    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
        this.Yolo();
    }
    public func Yolo() -> Void {
        let plugin = new AudiowarePlugin();
        plugin.Yolo();
    }
}