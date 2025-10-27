mod dto;
mod protocol;
mod udp;
use crate::dto::{paginate, Gateway, PageResponse, TagDto};
use crate::protocol::AoaGateway;
use lazy_static::lazy_static;
use local_ip_address::list_afinet_netifas;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::IpAddr;
use std::ops::{Div, Mul};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use std::{fmt, io, result};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, Runtime};
use tauri_plugin_udp::platform;
use time::{format_description, UtcDateTime};
use tokio::time::sleep;

lazy_static! {
    static ref GATEWAY_MAP: RwLock<HashMap<String, Arc<Mutex<Gateway>>>> =
        RwLock::new(HashMap::new());
}
lazy_static! {
    static ref GATEWAY_LIST: RwLock<Vec<Arc<Mutex<Gateway>>>> = RwLock::new(Vec::new());
}

#[tauri::command]
fn network_interfaces() -> Result<Vec<(String, IpAddr)>> {
    let network_interfaces = list_afinet_netifas()?;
    Ok(network_interfaces)
}
#[tauri::command]
async fn fetch_tag_list(
    gateway_mac: String,
    index: usize,
    size: usize,
    tag_mac: String,
) -> Result<PageResponse<Arc<Mutex<TagDto>>>> {
    let gateway_map = GATEWAY_MAP.read().unwrap();
    let tag_list = gateway_map
        .get(&gateway_mac)
        .unwrap()
        .lock()
        .unwrap()
        .tags
        .lock()
        .unwrap()
        .clone();
    let filter_data: Vec<_> = tag_list
        .iter()
        .filter(|item| item.lock().unwrap().mac.contains(&tag_mac))
        .map(|x| x.clone())
        .collect();
    let page_result = paginate(filter_data, index, size);
    Ok(page_result)
}
#[tauri::command]
async fn fetch_gateway(
    index: usize,
    size: usize,
    mac: String,
) -> Result<PageResponse<Arc<Mutex<Gateway>>>> {
    let gateway_list = GATEWAY_LIST.read().unwrap();
    let filter_data: Vec<_> = gateway_list
        .iter()
        .filter(|item| item.lock().unwrap().mac.contains(&mac))
        .map(|x| x.clone())
        .collect();
    let page_result = paginate(filter_data, index, size);
    Ok(page_result)
}
#[tauri::command]
async fn bind<R: Runtime>(
    window: tauri::Window<R>,
    id: String,
    ip: String,
    port: u16,
) -> Result<()> {
    GATEWAY_LIST.write().unwrap().clear();
    GATEWAY_MAP.write().unwrap().clear();
    udp::bind(window, id, ip, port, false).await?;

    Ok(())
}

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
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
#[tauri::command]
async fn unbind(id: String) -> Result<()> {
    udp::unbind(id).await?;
    Ok(())
}
#[warn(unused_must_use)]

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
#[tauri::command]
async fn export_tag_list(
    export_mode: i32,
    export_path: String,
    gateway_mac: String,
    tag_mac: String,
    yingshou: u32,
) -> Result<()> {
    let gateway_map = GATEWAY_MAP.read().unwrap();
    let gateway = gateway_map.get(&gateway_mac).unwrap().lock().unwrap();
    let mut file = OpenOptions::new()
        .create(true) // 不存在则创建
        .append(true) // 追加模式（不覆盖原有内容）
        .write(true) // 允许写入
        .open(export_path)?;

    if export_mode == 0 {
        writeln!(file,"Mac,电压,防拆,按钮,振动,心率,舒张压,收缩压,血氧,体温,计步,睡眠状态,深睡眠时间,浅睡眠时间,rssi,第一次上报时间,最后更新时间")?;
        let _ = &gateway.tag_packets.iter().filter(|item|{
            item.lock().unwrap().mac.contains(&tag_mac)
        }).for_each(|item|{
            let item = item.lock().unwrap();
            writeln!(file, "{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}, {:?}",
                     item.mac,
                     item.voltage.map_or(String::new(), |s| s.to_string()),
                     item.tamper.map_or(String::new(), |s| s.to_string()),
                     item.button.map_or(String::new(), |s| s.to_string()),
                     item.shock.map_or(String::new(), |s| s.to_string()),
                     item.heart_rate.map_or(String::new(), |s| s.to_string()),
                     item.blood_pressure_h.map_or(String::new(), |s| s.to_string()),
                     item.blood_pressure_l.map_or(String::new(), |s| s.to_string()),
                     item.blood_oxygen.map_or(String::new(), |s| s.to_string()),
                     item.body_temperature.map_or(String::new(), |s| s.to_string()),
                     item.step_count.map_or(String::new(), |s| s.to_string()),
                     item.sleep_state.map_or(String::new(), |s| s.to_string()),
                     item.deep_sleep_time.map_or(String::new(), |s| s.to_string()),
                     item.light_sleep_time.map_or(String::new(), |s| s.to_string()),
                     item.rssi,
                     format_time(item.last_time),
                     format_time(item.first_time)).expect("文件写入失败");
        });
    } else {
        writeln!(file,"Mac,电压,防拆,按钮,振动,心率,舒张压,收缩压,血氧,体温,计步,睡眠状态,深睡眠时间,浅睡眠时间,rssi,第一次上报时间,丢包率,最后更新时间")?;
        let _ = &gateway.tags.lock().unwrap().iter()
            .filter(|item|{
                item.lock().unwrap().mac.contains(&tag_mac)
            })
            .for_each(|item|{
            let item = item.lock().unwrap();
            writeln!(file, "{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
                     item.mac,
                     item.voltage.map_or(String::new(), |s| s.to_string()),
                     item.tamper.map_or(String::new(), |s| s.to_string()),
                     item.button.map_or(String::new(), |s| s.to_string()),
                     item.shock.map_or(String::new(), |s| s.to_string()),
                     item.heart_rate.map_or(String::new(), |s| s.to_string()),
                     item.blood_pressure_h.map_or(String::new(), |s| s.to_string()),
                     item.blood_pressure_l.map_or(String::new(), |s| s.to_string()),
                     item.blood_oxygen.map_or(String::new(), |s| s.to_string()),
                     item.body_temperature.map_or(String::new(), |s| s.to_string()),
                     item.step_count.map_or(String::new(), |s| s.to_string()),
                     item.sleep_state.map_or(String::new(), |s| s.to_string()),
                     item.deep_sleep_time.map_or(String::new(), |s| s.to_string()),
                     item.light_sleep_time.map_or(String::new(), |s| s.to_string()),
                     item.rssi,
                     format_time(item.last_time),
                    if yingshou!=0 { Decimal::from(yingshou-item.packet_count).div(Decimal::from(yingshou)).mul(Decimal::from(100)).round_dp_with_strategy(2,RoundingStrategy::MidpointAwayFromZero).to_f64().unwrap().to_string()+"%" } else { "0".to_string() } ,
                     format_time(item.first_time)).expect("文件写入失败");
        });
    }
    Ok(())
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
        ])
        .plugin(tauri_plugin_udp::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
