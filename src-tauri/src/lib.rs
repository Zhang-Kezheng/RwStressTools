mod dto;
mod protocol;
mod udp;
mod receive_command;
mod send_command;
use rust_decimal::prelude::ToPrimitive;
use std::error::Error;
use std::io::Write;
use std::ops::{Div, Mul};
use std::{io, result};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, Runtime};
use time::{format_description, UtcDateTime};
use crate::receive_command::{bind, export_tag_list, fetch_gateway, fetch_tag_list, network_interfaces, unbind};
use crate::send_command::{send_start, send_stop};

pub type Result<T> = result::Result<T, BusinessError>;
#[derive(Debug, thiserror::Error)]
pub enum BusinessError {
    #[error(transparent)]
    LocalIp(#[from] local_ip_address::Error),
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("{0}")]
    CUSTOM(String),
    #[error("json 序列化错误: {0}")]
    OTHER(#[from] serde_json::Error),
}

impl serde::Serialize for BusinessError {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn format_time(ms_timestamp: u128) -> String {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    // 转换为 DateTime
    UtcDateTime::from_unix_timestamp(ms_timestamp as i64)
        .unwrap()
        .format(&format)
        .unwrap()
        .to_string()
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_udp::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let window = tray.app_handle().get_webview_window("main").unwrap();
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            network_interfaces,
            bind,
            unbind,
            fetch_gateway,
            fetch_tag_list,
            export_tag_list,
            send_start,
            send_stop
        ])
        .plugin(tauri_plugin_udp::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
