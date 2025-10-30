use crate::receive_command::process;
use lazy_static::lazy_static;
use std::future::Future;
use std::{collections::HashMap, sync::Arc};
use tauri::Runtime;
use tokio::task::JoinHandle;
use tokio::{
    io,
    net::UdpSocket,
    sync::RwLock,
    time::{self, sleep},
};

lazy_static! {
    static ref SOCKETS: RwLock<HashMap<String, Udp>> = RwLock::new(HashMap::new());
}

pub async fn bind<R: Runtime>(
    window: tauri::Window<R>,
    id: String,
    ip: String,
    port: u16,
    broadcast: bool,
) -> io::Result<()> {
    let mut sockets = SOCKETS.write().await;

    if let Some(s) = sockets.get(&id) {
        s.task.abort();
        sockets.remove(&id);
        sleep(time::Duration::from_millis(100)).await;
    }
    let bind_at = format!("{}:{}", ip, port);
    let sock = UdpSocket::bind(bind_at.clone()).await?;
    sock.set_broadcast(broadcast)?;
    let arc = Arc::new(sock);
    let sock = arc.clone();
    println!("{} udp bond at {}", &id, bind_at.clone());
    let udp_id = id.clone();
    let task = tokio::task::spawn(async move {
        let mut buf = [0; 65535];
        loop {
            if let Ok((len, addr)) = sock.recv_from(&mut buf).await {
                if len == 1 && buf[0] == 0 {
                    break;
                }
                println!("{:?} bytes received from {:?}", len, addr);
                process(buf[..len].to_vec()).await;
            }
        }
        ()
    });

    sockets.insert(udp_id, Udp { task, sock: arc });
    Ok(())
}
pub struct Payload {
    id: String,
    addr: String,
    pub data: Vec<u8>,
}
struct Udp {
    task: JoinHandle<()>,
    sock: Arc<UdpSocket>,
}
pub async fn unbind(id: String) -> io::Result<()> {
    let mut sockets = SOCKETS.write().await;

    if let Some(s) = sockets.get(&id) {
        s.task.abort();
        sockets.remove(&id);
        println!("{} udp unbond", &id);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("ID {} not bond.", &id),
        ))
    }
}

pub async fn send(id: String, target: String, message: Vec<u8>) -> io::Result<()> {
    let sockets = SOCKETS.read().await;

    if let Some(s) = sockets.get(&id) {
        s.sock.send_to(&message, &target).await?;
        println!("{} udp sent {} bytes to {}", &id, message.len(), &target);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("ID {} not bond.", &id),
        ))
    }
}
