use bitsrun_lib::{SrunClient, SrunLoginState, get_login_state};
use log::{debug, warn};
use reqwest::Client;
use tauri::async_runtime::Mutex;
use tauri::menu::{CheckMenuItemBuilder, Menu, MenuBuilder, MenuItem, SubmenuBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager, State, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
#[cfg(not(dev))]
use tauri_plugin_prevent_default::{Flags, KeyboardShortcut};

use crate::api::ApiResponse;

mod api;

struct BitsguiState {
    client: Mutex<Option<SrunClient>>,
}

#[tauri::command]
async fn init(state: State<'_, BitsguiState>) -> Result<ApiResponse<String, String>, ()> {
    let client = SrunClient::new(Some(Client::new()), None, None).await;

    match client {
        Ok(client) => {
            *state.client.lock().await = Some(client);
            Ok(ApiResponse::success(
                "Client initialized successfully".into(),
            ))
        }
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn login(
    username: String,
    password: String,
    state: State<'_, BitsguiState>,
) -> Result<ApiResponse<String, String>, ()> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().unwrap();
    match client.login(false, username, password).await {
        Ok(response) => {
            debug!("{:?}", response);
            Ok(response
                .suc_msg
                .map(ApiResponse::success)
                .unwrap_or(ApiResponse::error(response.error)))
        }
        Err(e) => {
            debug!("{:?}", e);
            Ok(ApiResponse::error(e.to_string()))
        }
    }
}

#[tauri::command]
async fn logout(
    username: String,
    state: State<'_, BitsguiState>,
) -> Result<ApiResponse<String, String>, ()> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().unwrap();
    match client.logout(false, username).await {
        Ok(response) => {
            debug!("{:?}", response);
            if response.suc_msg.is_some() {
                Ok(ApiResponse::success(response.suc_msg.unwrap()))
            } else if response.error == "ok" {
                Ok(ApiResponse::success(response.error))
            } else {
                Ok(ApiResponse::error(response.error))
            }
        }
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn check_status() -> Result<ApiResponse<SrunLoginState, String>, ()> {
    let http_client = Client::new();

    match get_login_state(&http_client).await {
        Ok(resp) => Ok(ApiResponse::success(resp)),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

fn setup_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let autostart_manager = app.autolaunch();

    let item_auto_start = CheckMenuItemBuilder::new("自动启动")
        .id("auto_start")
        .checked(autostart_manager.is_enabled().unwrap_or(false))
        .build(app)?;

    let menu_settings = SubmenuBuilder::new(app, "设置")
        .item(&item_auto_start)
        .build()?;

    // let menu_dev =MenuItem::new(app, "设置")
    //     .item(&item_auto_start)
    //     .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&menu_settings])
        .text("devtools", "Devtools")
        .build()?;

    app.set_menu(menu)?;

    app.on_menu_event(
        move |app: &tauri::AppHandle, event| match event.id().0.as_str() {
            "auto_start" => {
                let autostart_manager = app.autolaunch();

                if item_auto_start.is_checked().unwrap() {
                    autostart_manager.enable().unwrap()
                } else {
                    autostart_manager.disable().unwrap()
                }
            }

            "devtools" => {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            _ => {
                println!("unexpected menu event");
            }
        },
    );

    Ok(())
}

fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let item_show_window = MenuItem::with_id(app, "show_window", "显示窗口", true, None::<&str>)?;
    let item_quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&item_show_window, &item_quit])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show_window" => {
                let app = app.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {
                warn!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::DoubleClick {
                id: _,
                position: _,
                rect: _,
                button: _,
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {
                println!("unhandled event {event:?}");
            }
        })
        .build(app)?;

    Ok(())
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .manage(BitsguiState {
            client: Mutex::new(None),
        })
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .level_for("reqwest", log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        );

    #[cfg(not(dev))]
    let builder = {
        let prevent = tauri_plugin_prevent_default::Builder::new()
            .with_flags(
                Flags::CONTEXT_MENU | Flags::PRINT | Flags::DOWNLOADS | Flags::RELOAD | Flags::FIND,
            )
            .shortcut(KeyboardShortcut::new("F12"))
            .build();
        builder.plugin(prevent)
    };

    builder
        .invoke_handler(tauri::generate_handler![init, login, logout, check_status,])
        .setup(|app| {
            setup_menu(app)?;
            setup_tray(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event
                && code.is_none()
            {
                api.prevent_exit();
            }
        });
}
