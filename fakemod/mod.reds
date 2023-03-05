module FakeMod

import Audioware.System.AudiowareSystem

@addMethod(PlayerPuppet)
public func PlayCustomSound(sound: String) -> Void {
 let system = AudiowareSystem.GetInstance(this.GetGame());
}