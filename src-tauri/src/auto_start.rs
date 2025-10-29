use tauri::{AppHandle, State};
use tauri_plugin_autostart::{AutoLaunchManager, Error, ManagerExt};
use tauri_plugin_store::StoreExt;

pub struct AutoStartManager<'r> {
    app: &'r AppHandle,
    manager: State<'r, AutoLaunchManager>,
}

impl<'r> AutoStartManager<'r> {
    pub fn new(app: &'r AppHandle) -> Self {
        let manager = app.autolaunch();

        Self { app, manager }
    }

    pub fn inherit_settings(&self) -> Result<(), Error> {
        let settings = self.app.store("settings").unwrap();
        let enabled = settings
            .get("auto_start")
            .unwrap_or_else(|| {
                settings.set("auto_start", false);
                settings.save().expect("failed to save settings");
                settings.get("auto_start").unwrap()
            })
            .as_bool()
            .unwrap();
        self.set_enabled(enabled)
    }

    pub fn set_enabled(&self, enable: bool) -> Result<(), Error> {
        if enable {
            self.manager.enable()
        } else {
            self.manager.disable()
        }
    }
}
