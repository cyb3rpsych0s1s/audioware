module Audioware

private native func RegisterEmitter(id: EntityID) -> Void;
private native func UnregisterEmitter(id: EntityID) -> Void;

public class Ventriloquist extends ScriptableComponent {
    let delay: DelayID;
    private final func OnGameAttach() -> Void {
        RegisterEmitter(this.GetEntity().GetEntityID());
    }
    private final func OnGameDetach() -> Void {
        UnregisterEmitter(this.GetEntity().GetEntityID());
    }
}

public static func AddVentriloquist(entity: Entity) -> Void {
    let component = new Ventriloquist();
    entity.AddComponent(component);
}

public static func FindEntityByID(gi: GameInstance, id: EntityID) -> ref<Entity> {
    return GameInstance.FindEntityByID(gi, id);
}
