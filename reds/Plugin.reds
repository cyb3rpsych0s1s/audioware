module Audioware

native func LoadBank(name: String) -> Void;

public class Plugin {
    private let handle: ref<FMOD>;
    public static func Initialize() -> ref<Plugin> {
        let plugin = new Plugin();
        plugin.handle = new FMOD();
        return plugin;
    }
    public static func Load(plugin: ref<Plugin>, bank: String) -> Void {
        plugin.handle.Load(bank)
    }
}

private class FMOD {
    public func Load(bank: String) -> Void {
        LoadBank(bank);
    }
}

public class Bank {
    let filenames: array<String>;
    public func Get() -> ref<array<String>> = this.filenames;
}
