module Audioware

@wrapMethod(SettingsMainGameController)
protected cb func OnButtonRelease(evt: ref<inkPointerEvent>) -> Bool {
    if evt.IsAction(n"run_benchmark") && this.IsBenchmarkPossible() && inkWidgetRef.IsVisible(this.m_benchmarkButton) {
        SetInBenchmark(true);
    }
    return wrappedMethod(evt);
}

@wrapMethod(SettingsMainGameController)
protected cb func OnBenchmarkButtonReleased(controller: wref<inkButtonController>) -> Bool {
    if this.IsBenchmarkPossible() {
        SetInBenchmark(true);
    }
    return wrappedMethod(controller);
}

@wrapMethod(MenuScenario_BenchmarkResults)
protected cb func OnBenchmarkResultsClose() -> Bool {
    SetInBenchmark(false);
    return wrappedMethod();
}