use dioxus::desktop::trayicon::TrayIconEvent;
use dioxus::desktop::use_tray_icon_event_handler;

use super::show_window;

pub fn use_tray_activation() {
    use_tray_icon_event_handler(move |event| {
        if let TrayIconEvent::DoubleClick { .. } = event {
            show_window();
        }
    });
}
