use dioxus::desktop::trayicon::{MouseButton, MouseButtonState, TrayIconEvent};
use dioxus::desktop::use_tray_icon_event_handler;

use super::show_window;

pub fn use_tray_activation() {
    use_tray_icon_event_handler(move |event| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Down,
            ..
        } = event
        {
            show_window();
        }
    });
}
