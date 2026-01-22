import Audioware.*

public class ChangeMenuSounds extends ScriptableService {
    private let mainMenuMusic: ref<DynamicSoundEvent>;
    private let isPreGame: Bool = false;
    private cb func OnLoad() {
        if !EnableAudiowareChangeMenuSounds() { return; }
        let manager = new AudioEventManager();
        // mute main menu ambience music (once clicked Press to Start)
        manager.Mute(n"mus_game_menus_00_title_screen_START");
        
        GameInstance.GetCallbackSystem()
              .RegisterCallback(n"Session/Start", this, n"OnSessionStart");
        // listen for main menu entered (once clicked Press to Start)
        GameInstance.GetCallbackSystem()
            .RegisterCallback(n"InkWidget/Spawn", this, n"OnInitialLoadingControllerSupervisorSpawn")
            .AddTarget(inkWidgetTarget.Controller(n"inkInitialLoadingControllerSupervisor"));
    }
    private cb func OnSessionStart(event: ref<GameSessionEvent>) {
        this.mainMenuMusic = null;
        let manager = new AudioEventManager();
        if event.IsPreGame() {
            // mute main menu UI sounds when pre-game
            manager.Mute(n"ui_menu_hover");
            manager.Mute(n"ui_menu_onpress");
        } else {
            // unmute main menu UI sounds while in-game
            manager.Unmute(n"ui_menu_hover");
            manager.Unmute(n"ui_menu_onpress");
        }
        this.isPreGame = event.IsPreGame();
    }
    // when entering main menu, play another ambience music
    private cb func OnInitialLoadingControllerSupervisorSpawn(event: ref<inkWidgetSpawnEvent>) {
        // listen for main menu sounds
        GameInstance.GetAudioEventCallbackSystem()
            .RegisterCallback(n"ui_menu_hover", this, n"OnUIMenuHover");
        GameInstance.GetAudioEventCallbackSystem()
            .RegisterCallback(n"ui_menu_onpress", this, n"OnUIMenuOnPress");
        
        if IsDefined(this.mainMenuMusic) { return; }
            
        // pick another theme :)
        let settings = new AudioSettingsExt();
        settings.loop = true; // make it loop
        this.mainMenuMusic = DynamicSoundEvent.Create(n"guile_theme", settings);
        // effectively enqueue and play it
        event
            .GetItemInstance()
            .gameController
            .QueueEvent(this.mainMenuMusic);
    }
    // on main menu hover, change sound
    private cb func OnUIMenuHover(event: ref<SoundEvent>) {
        if !this.isPreGame { return; }
        let system = new AudioSystemExt();
        system.Play(n"sf_on_hover");
    }
    // on main menu press, change sound
    private cb func OnUIMenuOnPress(event: ref<SoundEvent>) {
        if !this.isPreGame { return; }
        let system = new AudioSystemExt();
        system.Play(n"sf_on_press");
    }
}