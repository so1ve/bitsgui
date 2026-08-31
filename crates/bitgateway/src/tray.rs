#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use dioxus::desktop::trayicon::menu::{Menu, MenuItem, PredefinedMenuItem};
use dioxus::desktop::trayicon::{DioxusTrayIcon, init_tray_icon};
use dioxus::desktop::{WindowCloseBehaviour, icon_from_memory, use_muda_event_handler, window};
use dioxus::prelude::*;

const MENU_SHOW_WINDOW: &str = "show_window";
const MENU_EXIT_APP: &str = "exit_app";
const TRAY_ICON: &[u8] = include_bytes!("../assets/icons/tray.png");

pub fn use_tray() {
    use_hook(|| {
        let show_item = MenuItem::with_id(MENU_SHOW_WINDOW, "显示窗口", true, None);
        let exit_item = MenuItem::with_id(MENU_EXIT_APP, "退出程序", true, None);
        let separator = PredefinedMenuItem::separator();
        let tray_menu = Menu::with_items(&[&show_item, &separator, &exit_item]).unwrap();
        let tray_icon = icon_from_memory::<DioxusTrayIcon>(TRAY_ICON).unwrap();

        init_tray_icon(tray_menu, Some(tray_icon));
    });

    use_muda_event_handler(move |event| match event.id().as_ref() {
        MENU_SHOW_WINDOW => show_window(),
        MENU_EXIT_APP => exit_app(),
        _ => {}
    });

    #[cfg(target_os = "macos")]
    macos::use_tray_activation();

    #[cfg(target_os = "windows")]
    windows::use_tray_activation();
}

fn show_window() {
    let desktop = window();
    desktop.set_visible(true);
    desktop.set_minimized(false);
    desktop.set_focus();
}

fn exit_app() {
    let desktop = window();
    desktop.set_close_behavior(WindowCloseBehaviour::WindowCloses);
    desktop.close();
}
