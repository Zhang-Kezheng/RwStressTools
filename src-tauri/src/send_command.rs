use std::time::Duration;
use bytebuffer::ByteBuffer;
use lazy_static::lazy_static;
use rand::{Rng, RngCore};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc};
use tokio::task::{JoinError, JoinHandle, JoinSet};
use tokio::time::sleep;
use std::sync::{Arc, Mutex, Condvar, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use log::log;
use rand::rngs::ThreadRng;
use crate::BusinessError;
use crate::client::{Client, TcpClient, UdpClient};
use crate::dto::Gateway;
use crate::protocol::{AoaGateway, AoaTag};
static  RUNNING:AtomicBool = AtomicBool::new(false);
//0 udp 1 tcp
#[tauri::command]
pub async fn send_start(
    protocol:i32,
    target: String,
    thread_count: i32,
    rate: u64,
) -> crate::Result<()> {
    let  duration = Duration::from_millis(rate); //
    let mut join_set=JoinSet::new();
    RUNNING.store(true, Ordering::SeqCst);
    for index in 0..thread_count {
        let value = target.clone();
        join_set.spawn(async move{
            let mut client:Box<dyn Client + Send> = if protocol==0 {Box::new( UdpClient::new(value).await.expect("udp客户端创建失败")) } else { Box::new(TcpClient::new(value).await.expect("tcp客户端创建失败")) };
            while RUNNING.load(Ordering::SeqCst)  {
                let data=build_packet(index);
                client.write( data.as_slice()).await.expect("couldn't send data");
                sleep(duration).await; // 模拟耗时操作
            }
            client.stop().await.expect("停止失败");
        });
    }
    while let Some(result) = join_set.join_next().await {
        return match result {
            // 任务执行时发生 panic（如 unwrap 失败）
            Err(join_err) => {
                RUNNING.store(false, Ordering::SeqCst);
                join_set.abort_all();
                Err(BusinessError::CUSTOM(join_err.to_string()))
            }
            // 任务返回业务结果
            Ok(id) =>  {
                Ok(())
            },
        }
    }
    Ok(())
}
#[tauri::command]
pub async fn send_stop(
) -> crate::Result<()> {
    RUNNING.store(false, Ordering::SeqCst);
    Ok(())
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
    let tag_count=26;
    let mut rng =rand::rng();
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