use std::time::Duration;
use bytebuffer::ByteBuffer;
use lazy_static::lazy_static;
use rand::{Rng, RngCore};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use log::log;
use crate::dto::Gateway;
use crate::protocol::{AoaGateway, AoaTag};

lazy_static! {
    static ref SEND_TASK_LIST: RwLock<Vec<JoinHandle<()>>> = RwLock::new(Vec::new());
}
#[tauri::command]
pub async fn send_start(
    target: String,
    thread_count: i32,
    rate: u64,
) -> crate::Result<()> {
    for index in 0..thread_count {
        SEND_TASK_LIST.write().await.push(send_task(target.clone(),rate,index));
    }
    Ok(())
}
#[tauri::command]
pub async fn send_stop(
) -> crate::Result<()> {
    SEND_TASK_LIST.read().await.iter().for_each(|x| {
       let _=x.abort();
    });
    SEND_TASK_LIST.write().await.clear();
    Ok(())
}
 fn send_task(target: String,rate: u64,index:i32)->JoinHandle<()>{
    let  duration = Duration::from_millis(rate); // 设置为每小时执行一次
    tokio::spawn( async move {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        loop {
            let data=build_packet(index);
            println!("{}",data.len());
            socket.send_to(data.as_slice(), target.as_str()).await.expect("couldn't send data");
            sleep(duration).await; // 模拟耗时操作
        }
    })

}
lazy_static!{
    static ref MAC_CONTAINER: Vec<[u8;6]> ={
      let mut container: Vec<[u8;6]> =vec![];
        for i in 0..10000 {
            let mut buffer = ByteBuffer::new();
            buffer.write_u8(0x02);
            buffer.write_u8(0x01);
            buffer.write_i32(i);
            let mac:[u8;6]=buffer.as_bytes().try_into().unwrap();
            container.push(mac);
        }
        container
    };
}
fn build_packet(index:i32)-> Vec<u8>{
    let mut rng = rand::rng();
    let tag_count=26;
    let mut buffer = ByteBuffer::new();
    buffer.write_u8(tag_count);
    for i in 0..26 {
        let mut user_data:[u8;3]=[0,0,0];
        rng.fill_bytes(user_data.as_mut_slice());
        let aoa_tag=AoaTag{
            mac:MAC_CONTAINER[rng.random_range(0..10000)],
            length:0x1e,
            fix:0xff,
            manufacturer_id:0x0d00,
            package_id:0x04,
            command:rng.random_range(0x09..0x0f),
            user_data,
            crc:0x00,
            df_field:[
                0x2F,0x61,0xAC,0xCC,0x27,0x45,0x67,0xF7,0xDB
                ,0x34,0xC4,0x03,0x8E,0x5C,0x0B,0xAA,0x97,0x30,0x56,0xE6
            ],
            rssi:0x04
        };
        buffer.write_bytes(aoa_tag.to_bytes().as_slice());
    }
    let mut buffer_gateway = ByteBuffer::new();
    buffer_gateway.write_u8(0x01);
    buffer_gateway.write_u8(0x02);
    buffer_gateway.write_i32(index);
    let device_id:[u8;6]=buffer_gateway.as_bytes().try_into().unwrap();
    let aoa_gateway=AoaGateway::new(buffer.as_bytes(),device_id);
    aoa_gateway.to_bytes()
}