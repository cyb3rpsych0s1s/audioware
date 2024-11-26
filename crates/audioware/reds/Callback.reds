module Audioware

public class HideSubtitleCallback extends DelayCallback {
    public let line: scnDialogLineData;
    public func Call() -> Void {
        if !IsDefined(this.line.speaker) { return; }
        let game = this.line.speaker.GetGame();
        let id = SubtitleSubSystem.GetInstance(game).subtitleDelayID;
        GameInstance
        .GetDelaySystem(game)
        .CancelCallback(id);
        let board: ref<IBlackboard> = GameInstance.GetBlackboardSystem(game).Get(GetAllBlackboardDefs().UIGameData);
        board.SetVariant(GetAllBlackboardDefs().UIGameData.HideDialogLine, [this.line.id], true);
    }
}