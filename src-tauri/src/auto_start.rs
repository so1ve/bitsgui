use std::sync::Arc;

use tauri::{State, Wry};
use tauri_plugin_autostart::{AutoLaunchManager, Error, ManagerExt};
use tauri_plugin_store::{Store, StoreExt};

pub struct AutoStartManager<'r> {
    manager: State<'r, AutoLaunchManager>,
    settings: Arc<Store<Wry>>,
}

impl<'r> AutoStartManager<'r> {
    pub fn new<A>(app: &'r A) -> Self
    where
        A: ManagerExt<Wry> + StoreExt<Wry>,
    {
        let manager = app.autolaunch();
        let settings = app.store("settings").unwrap();

        Self { manager, settings }
    }

    pub fn inherit_settings(&self) -> Result<(), Error> {
        self.set_enabled(self.is_enabled_in_config())
    }

    pub fn set_enabled(&self, enable: bool) -> Result<(), Error> {
        if enable {
            self.manager.enable()
        } else {
            // 系统找不到指定的文件。 (os error 2) Bug workaround: create registry entry
            // before disabling
            self.manager.enable()?;
            self.manager.disable()
        }
        .map(|_| {
            self.settings.set("auto_start", enable);
            self.settings.save().expect("failed to save settings");
        })
    }

    pub fn is_enabled_in_config(&self) -> bool {
        self.settings
            .get("auto_start")
            .unwrap_or_else(|| {
                self.settings.set("auto_start", false);
                self.settings.save().expect("failed to save settings");
                self.settings.get("auto_start").unwrap()
            })
            .as_bool()
            .unwrap()
    }
}
