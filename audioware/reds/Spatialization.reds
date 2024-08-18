module Audioware

public class SpatializationSystem extends ScriptableSystem {
    private func OnAttach() -> Void {
        LOG("on attach: SpatializationSystem");
    }
    private func OnDetach() -> Void {
        LOG("on detach: SpatializationSystem");
    }
}
public native class ExtSystem extends IScriptableSystem {
    private func OnAttach() -> Void {
        LOG("on attach: SpatializationSystem");
    }
    private func OnDetach() -> Void {
        LOG("on detach: SpatializationSystem");
    }
}