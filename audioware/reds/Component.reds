module Audioware

public class Ventriloquist extends ScriptableComponent {
    let delay: DelayID;
    let registered: Bool;
    private let owner: wref<GameObject>;
    private let owner_id: EntityID;
    private final func OnGameAttach() -> Void {
        LogChannel(n"DEBUG", "[Ventriloquist] on game attach");
        this.owner = this.GetOwner();
        this.owner_id = this.owner.GetEntityID();
    }
    private final func OnGameDetach() -> Void {
        LogChannel(n"DEBUG", "[Ventriloquist] on game detach");
        this.Unregister();
    }
    public func Register(id: EntityID) -> Void {
        LogChannel(n"DEBUG", s"[Ventriloquist] Register EntityID: provided \(EntityID.ToDebugString(id)), inner \(EntityID.ToDebugString(this.owner_id))");
        if !this.registered {
            this.owner_id = id;
            this.owner = GameInstance.FindEntityByID(GetGameInstance(), id) as GameObject;
            if !IsDefined(this.owner) { LogChannel(n"DEBUG", "no owner !!!"); return; }
            this.registered = true;
            let system = Audioware.GetInstance(GetGameInstance());
            system.RegisterVentriloquist(id);
            LogChannel(n"DEBUG", s"register \(EntityID.ToDebugString(this.owner_id))");
        }
    }
    public func Unregister() -> Void {
        LogChannel(n"DEBUG", s"[Ventriloquist] Unregister EntityID: inner \(EntityID.ToDebugString(this.owner_id))");
        if this.registered {
            this.registered = false;
            let system = Audioware.GetInstance(GetGameInstance());
            system.UnregisterVentriloquist(this.owner_id);
            LogChannel(n"DEBUG", s"unregister \(EntityID.ToDebugString(this.owner_id))");
        }
    }
}

public static func FindEntityByID(gi: GameInstance, id: EntityID) -> ref<Entity> {
    return GameInstance.FindEntityByID(gi, id);
}
