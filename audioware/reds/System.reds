module Audioware

public class Audioware extends ScriptableSystem {
    private let m_callbackSystem: wref<CallbackSystem>;

    private func OnAttach() {
        this.m_callbackSystem = GameInstance.GetCallbackSystem();
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Start", this, n"OnSessionStart");
        this.m_callbackSystem.RegisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.RegisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
    }

    private func OnDetach() {
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeStart", this, n"OnSessionBeforeStart");
        this.m_callbackSystem.UnregisterCallback(n"Session/Ready", this, n"OnSessionReady");
        this.m_callbackSystem.UnregisterCallback(n"Session/BeforeEnd", this, n"OnSessionBeforeEnd");
        this.m_callbackSystem = null;
    }

    private cb func OnSessionBeforeStart(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.Start);
    }
    private cb func OnSessionReady(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.InGame);
    }
    private cb func OnSessionBeforeEnd(event: ref<GameSessionEvent>) {
        UpdateEngineState(EngineState.End);
    }
}