use crate::client::{Client, TcpClient, UdpClient};
use crate::dto::{paginate, Gateway, PageResponse, TagDto};
use crate::protocol::{AoaGateway, AoaTag};
use crate::server::{Server, TcpServer, UdpServer};
use bytebuffer::ByteBuffer;
use lazy_static::lazy_static;
use local_ip_address::list_afinet_netifas;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::ops::{Div, Mul};
use std::sync::{Arc};
use std::time::Duration;
use tauri::{Emitter, Manager, Runtime};
use std::sync::{Mutex,RwLock};
use chrono::{DateTime, TimeZone, Utc};
use time::{format_description, UtcDateTime};
use tokio::task::JoinHandle;
use tokio::time::sleep;

lazy_static! {
    static ref GATEWAY_MAP: RwLock<HashMap<String, Arc<Mutex<Gateway>>>> =
        RwLock::new(HashMap::new());
}
lazy_static! {
    pub static ref GATEWAY_LIST: RwLock<Vec<Arc<Mutex<Gateway>>>> = RwLock::new(Vec::new());
}
lazy_static! {
    static ref SERVER: RwLock<Option<Arc<dyn Server + Send + Sync>>> = RwLock::new(None);
}

#[tauri::command]
pub fn network_interfaces() -> crate::Result<Vec<(String, IpAddr)>> {
    let network_interfaces = list_afinet_netifas()?;
    Ok(network_interfaces)
}
#[tauri::command]
pub async fn fetch_tag_list(
    gateway_mac: String,
    index: usize,
    size: usize,
    tag_mac: String,
) -> crate::Result<PageResponse<Arc<Mutex<TagDto>>>> {
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
        .filter( |item| item.lock().unwrap().mac.to_lowercase().contains(&tag_mac.to_lowercase()))
        .map(|x| x.clone())
        .collect();
    let page_result = paginate(filter_data, index, size);
    Ok(page_result)
}
#[tauri::command]
pub async fn fetch_gateway(
    index: usize,
    size: usize,
    mac: String,
) -> crate::Result<PageResponse<Arc<Mutex<Gateway>>>> {
    let gateway_list = GATEWAY_LIST.read().unwrap();
    let filter_data: Vec<_> = gateway_list
        .iter()
        .filter( |item| item.lock().unwrap().mac.contains(&mac))
        .map(|x| x.clone())
        .collect();
    let page_result = paginate(filter_data, index, size);
    Ok(page_result)
}
fn format_time(ms_timestamp: i64) -> String {

    // 1. 格式化 UTC 时间（推荐跨时区场景）
    let utc_dt: DateTime<Utc> = Utc.timestamp_opt(ms_timestamp , 0).unwrap();
     utc_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
#[tauri::command]
pub async fn receive_start(protocol: i32, ip: String, port: u16) -> crate::Result<()> {
    GATEWAY_LIST.write().unwrap().clear();
    GATEWAY_MAP.write().unwrap().clear();
    let bind_at = format!("{}:{}", ip, port);
    let task = tokio::spawn(async {
        let duration = Duration::from_secs(1); // 设置为每小时执行一次
        loop {
            GATEWAY_LIST
                .read()
                .unwrap()
                .iter()
                .for_each( |item| item.lock().unwrap().packet_receive_rate = 0);
            sleep(duration).await; // 模拟耗时操作
        }
    });
    let server: Arc<dyn Server + Send + Sync> = if protocol == 0 {
        Arc::new(
            UdpServer::bind(bind_at, task)
                .await
                .expect("UDP服务器创建失败"),
        )
    } else {
        Arc::new(
            TcpServer::bind(bind_at, task)
                .await
                .expect("TCP服务器创建失败"),
        )
    };
    *SERVER.write().unwrap() = Some(server.clone());

    Ok(())
}

#[tauri::command]
pub async fn receive_stop() -> crate::Result<()> {
    if SERVER.read().unwrap().is_some() {
        let _ = SERVER.read().unwrap().as_ref().unwrap().stop();
        *SERVER.write().unwrap() = None
    }
    Ok(())
}
#[tauri::command]
pub async fn export_tag_list(
    export_mode: i32,
    export_path: String,
    gateway_mac: String,
    tag_mac: String,
    yingshou: u32,
) -> crate::Result<()> {
    let gateway_map = GATEWAY_MAP.read().unwrap();
    let gateway = gateway_map.get(&gateway_mac).unwrap().lock().unwrap();
    let mut file = OpenOptions::new()
        .create(true) // 不存在则创建
        .append(true) // 追加模式（不覆盖原有内容）
        .write(true) // 允许写入
        .open(export_path)?;

    if export_mode == 0 {
        writeln!(file,"Mac,电压,防拆,按钮,振动,心率,舒张压,收缩压,血氧,体温,计步,睡眠状态,深睡眠时间,浅睡眠时间,rssi,第一次上报时间,最后更新时间")?;
        let _ = &gateway.tag_packets.iter().filter( |item|{
            item.lock().unwrap().mac.contains(&tag_mac)
        }).for_each( |item|{
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

pub async fn process(data: Vec<u8>) {
    if let Some(aoa_gateway) = AoaGateway::get_instance(data) {
        let mac = crate::dto::format_mac(aoa_gateway.dev_id);
        let mut byte_buffer = ByteBuffer::from_vec(aoa_gateway.data.clone());
        let count = byte_buffer.read_u8().unwrap() as usize;
        let tag_list = Mutex::new(vec![]);
        let tag_map = Mutex::new(HashMap::new());
        if aoa_gateway.data.len() == count * 38 + 1 {
            for _i in 0..count {
                let aoa_tag = AoaTag::get_instance(
                    byte_buffer
                        .read_bytes(38)
                        .unwrap()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                );
                let tag = Arc::new(Mutex::new(crate::dto::transform(aoa_tag)));
                tag_list.lock().unwrap().push(tag.clone());
                tag_map
                    .lock()
                    .unwrap()
                    .insert(tag.lock().unwrap().mac.clone(), tag.clone());
            }
        }
        let mut gateway_list = GATEWAY_LIST.write().unwrap();
        let mut gateway_map = GATEWAY_MAP.write().unwrap();
        if gateway_map.contains_key(&mac) {
            let mut gateway = gateway_map.get(&mac).unwrap().lock().unwrap();
            gateway.packet_receive_rate += tag_list.lock().unwrap().len().to_u32().unwrap() * 38;
            gateway.total += tag_list.lock().unwrap().len().to_u32().unwrap();
            for tag in tag_list.lock().unwrap().iter() {
                let tag_guard = tag.clone();
                gateway.tag_packets.push(tag.clone());
                let mut tag_map = gateway.tag_map.lock().unwrap();
                if tag_map.contains_key(&tag_guard.lock().unwrap().mac) {
                    let item = tag_map.get_mut(&tag_guard.lock().unwrap().mac).unwrap();
                    item.lock().unwrap().merge(&tag_guard.lock().unwrap());
                    item.lock().unwrap().packet_count += 1;
                    //合并
                } else {
                    tag_map.insert(tag.lock().unwrap().mac.clone(), tag.clone());
                    gateway.tags.lock().unwrap().push(tag.clone());
                }
            }
        } else {
            let tag_list_clone = tag_list.lock().unwrap().clone();
            let gateway = Arc::new(Mutex::new(Gateway {
                mac: mac.clone(),
                total: tag_list_clone.len() as u32,
                tags: tag_list,
                packet_receive_rate: 0,
                tag_map,
                tag_packets: tag_list_clone,
            }));
            gateway_list.push(gateway.clone());
            gateway_map.insert(mac, gateway.clone());
        }
    }
}
