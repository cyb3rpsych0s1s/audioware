module Audioware

public native func CallYoloOn(system: ref<AudiowareSystem>) -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
    }
    public func Yolo() -> Void {
        FTLog(AsRef("yololololo"));
    }
}