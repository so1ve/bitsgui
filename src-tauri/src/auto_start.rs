use std::sync::Arc;

use tauri::{AppHandle, State, Wry};
use tauri_plugin_autostart::{AutoLaunchManager, Error, ManagerExt};
use tauri_plugin_store::{Store, StoreExt};

pub struct AutoStartManager<'r> {
    app: &'r AppHandle,
    manager: State<'r, AutoLaunchManager>,
    settings: Arc<Store<Wry>>,
}

impl<'r> AutoStartManager<'r> {
    pub fn new(app: &'r AppHandle) -> Self {
        let manager = app.autolaunch();
        let settings = app.store("settings").unwrap();

        Self {
            app,
            manager,
            settings,
        }
    }

    pub fn inherit_settings(&self) -> Result<(), Error> {
        let enabled = self
            .settings
            .get("auto_start")
            .unwrap_or_else(|| {
                self.settings.set("auto_start", false);
                self.settings.save().expect("failed to save settings");
                self.settings.get("auto_start").unwrap()
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
        .map(|_| {
            self.settings.set("auto_start", enable);
            self.settings.save().expect("failed to save settings");
        })
    }
}
