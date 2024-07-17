module Audioware

public native func TestPlay() -> Void;

public class AudiowareSystem extends ScriptableSystem {
    private func OnAttach() -> Void {
        FTLog(AsRef("on attach: AudiowareSystem"));
    }
    private final func OnPlayerAttach(request: ref<PlayerAttachRequest>) -> Void {
        FTLog(AsRef("on player attach: AudiowareSystem"));
        TestPlay();
    }
    private final func OnPlayerDetach(request: ref<PlayerDetachRequest>) -> Void {
        FTLog(AsRef("on player detach: AudiowareSystem"));
        UnsetPlayerGender();
    }
}
