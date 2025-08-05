module Audioware

public class HideSubtitleCallback extends DelayCallback {
    public let line: scnDialogLineData;
    public func Call() -> Void {
        if !IsDefined(this.line.speaker) { return; }
        let game = this.line.speaker.GetGame();
        SubtitleSubSystem.GetInstance(game).CancelDelay();
        let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
        board.SetVariant(GetAllBlackboardDefs().UIGameData.HideDialogLine, [this.line.id], true);
    }
}