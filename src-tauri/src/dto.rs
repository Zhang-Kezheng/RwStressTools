use crate::protocol::{AoaGateway, AoaTag};
use crate::udp::Payload;
use bytebuffer::ByteBuffer;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::ops::{Div, Mul};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Emitter, Manager, Runtime};

// 导入必要的 trait（如 FromStr）
#[derive(Debug, Serialize, Clone)]
pub struct TagDto {
    pub mac: String,
    pub voltage: Option<f64>,
    pub tamper: Option<bool>,
    pub button: Option<bool>,
    pub shock: Option<bool>,
    pub heart_rate: Option<u8>,
    pub blood_pressure_h: Option<u8>,
    pub blood_pressure_l: Option<u8>,
    pub blood_oxygen: Option<u8>,
    pub body_temperature: Option<u8>,
    pub step_count: Option<u16>,
    pub sleep_state: Option<u8>,
    pub deep_sleep_time: Option<u8>,
    pub light_sleep_time: Option<u8>,
    pub rssi: i32,
    pub last_time: u128,
    pub first_time: u128,
    pub packet_count: u32,
}

impl TagDto {
    pub(crate) fn merge(&mut self, other: &TagDto) {
        if let Some(voltage) = other.voltage {
            self.voltage = Some(voltage);
        }
        if let Some(tamper) = other.tamper {
            self.tamper = Some(tamper);
        }
        if let Some(button) = other.button {
            self.button = Some(button);
        }
        if let Some(shock) = other.shock {
            self.shock = Some(shock);
        }
        if let Some(heart_rate) = other.heart_rate {
            self.heart_rate = Some(heart_rate);
        }
        if let Some(blood_pressure_h) = other.blood_pressure_h {
            self.blood_pressure_h = Some(blood_pressure_h);
        }
        if let Some(blood_pressure_l) = other.blood_pressure_l {
            self.blood_pressure_l = Some(blood_pressure_l);
        }
        if let Some(blood_oxygen) = other.blood_oxygen {
            self.blood_oxygen = Some(blood_oxygen);
        }
        if let Some(body_temperature) = other.body_temperature {
            self.body_temperature = Some(body_temperature);
        }
        if let Some(step_count) = other.step_count {
            self.step_count = Some(step_count);
        }
        if let Some(sleep_state) = other.sleep_state {
            self.sleep_state = Some(sleep_state);
        }
        if let Some(deep_sleep_time) = other.deep_sleep_time {
            self.deep_sleep_time = Some(deep_sleep_time);
        }
        if let Some(light_sleep_time) = other.light_sleep_time {
            self.light_sleep_time = Some(light_sleep_time);
        }
        self.rssi = other.rssi;
        self.last_time = other.last_time;
    }
}
#[derive(Debug)]
pub struct Gateway {
    pub(crate) mac: String,
    pub(crate) total: u32,
    pub(crate) packet_receive_rate: u32,
    pub(crate) tags: Mutex<Vec<Arc<Mutex<TagDto>>>>,
    pub(crate) tag_map: Mutex<HashMap<String, Arc<Mutex<TagDto>>>>,
    pub(crate) tag_packets: Vec<Arc<Mutex<TagDto>>>,
}
impl Serialize for Gateway {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct GatewayDto {
            mac: String,
            total: u32,
            tag_count: usize,
            packet_receive_rate: u32,
        }
        // 序列化临时结构体
        GatewayDto {
            tag_count: self.tags.lock().unwrap().len(),
            mac: self.mac.clone(),
            total: self.total,
            packet_receive_rate: self.packet_receive_rate,
        }
        .serialize(serializer)
    }
}
pub(crate) fn transform(aoa_tag: AoaTag) -> TagDto {
    let mut tag_dto = TagDto {
        mac: format_mac(aoa_tag.mac),
        voltage: None,
        tamper: None,
        button: None,
        shock: None,
        heart_rate: None,
        blood_pressure_h: None,
        blood_pressure_l: None,
        blood_oxygen: None,
        body_temperature: None,
        step_count: None,
        sleep_state: None,
        deep_sleep_time: None,
        light_sleep_time: None,
        rssi: aoa_tag.rssi as i32,
        last_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        first_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        packet_count: 1,
    };
    match aoa_tag.command {
        0x09 => {
            tag_dto.voltage = (Decimal::from_u8(aoa_tag.user_data[2])
                .unwrap()
                .mul(Decimal::from_str("6.6").unwrap())
                / Decimal::from(255))
            .round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp)
            .to_f64();
            tag_dto.tamper = Some(((aoa_tag.user_data[0] >> 5) & 0x01) == 1);
            tag_dto.button = Some(((aoa_tag.user_data[0] >> 4) & 0x01) == 1);
            tag_dto.shock = Some(((aoa_tag.user_data[0] >> 3) & 0x01) == 1);
        }
        0x0A => {
            tag_dto.heart_rate = Some(aoa_tag.user_data[0]);
            tag_dto.blood_pressure_h = Some(aoa_tag.user_data[1]);
            tag_dto.blood_pressure_l = Some(aoa_tag.user_data[2]);
        }
        0x0B => {
            tag_dto.blood_oxygen = Some(aoa_tag.user_data[0]);
        }
        0x0C => {
            let mut byte_buffer = ByteBuffer::from_bytes(&aoa_tag.user_data);
            tag_dto.body_temperature = Some(byte_buffer.read_u8().unwrap());
            tag_dto.step_count = Some(byte_buffer.read_u16().unwrap());
        }
        0x0D => {
            tag_dto.sleep_state = Some(aoa_tag.user_data[0]);
            tag_dto.light_sleep_time = Some(aoa_tag.user_data[1]);
            tag_dto.deep_sleep_time = Some(aoa_tag.user_data[2]);
        }
        _ => {}
    }
    tag_dto
}

#[derive(Debug, Serialize, Clone)]
pub struct PageResponse<T> {
    data: Vec<T>,
    total: usize,
}
pub fn paginate<T: Clone>(data: Vec<T>, page_index: usize, page_size: usize) -> PageResponse<T> {
    if page_size == 0 {
        return PageResponse {
            data: vec![],
            total: 0,
        };
    }
    let start = (page_index - 1) * page_size; // 计算起始索引
    let end = start + page_size;
    // 截取切片（自动处理越界，超出范围时返回空）
    PageResponse {
        data: data[start..end.min(data.len())].to_vec(),
        total: data.len(),
    }
}
pub(crate) fn format_mac(bytes: [u8; 6]) -> String {
    let mut hex_str = String::with_capacity(bytes.len() * 2); // 预分配容量，优化性能
    for byte in bytes {
        // 格式化每个字节为两位 16 进制（0-255 -> "00"-"ff"）
        hex_str.push_str(&format!("{:02x}", byte));
    }
    hex_str
}
