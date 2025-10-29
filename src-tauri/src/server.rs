use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpSocket, TcpStream, UdpSocket};
use tokio::task::JoinHandle;
use crate::receive_command::process;

#[async_trait]
pub trait Server{

    fn stop(&self);
}


pub struct UdpServer{
    udp_socket: Arc<UdpSocket>,
    receive_task:JoinHandle<()>,
    task:JoinHandle<()>
}
impl UdpServer{
    pub(crate) async fn bind(bind_at: String, task:JoinHandle<()>) -> crate::Result<Self>
    {
        println!("UDP 服务器已启动，监听端口 {}...",bind_at);
        let udp_socket = UdpSocket::bind(bind_at).await?;
        let udp_socket= Arc::new(udp_socket);
        let udp_socket_clone=udp_socket.clone();
        let receive_task = tokio::task::spawn(async move {
            let mut buf = [0; 65535];
            loop {
                if let Ok((len, addr)) = udp_socket_clone.recv_from(&mut buf).await {
                    if len == 1 && buf[0] == 0 {
                        break;
                    }
                    process(buf[..len].to_vec()).await;
                }
            }
            ()
        });
        Ok(UdpServer{udp_socket,receive_task,task})
    }
}
impl Server for UdpServer{

    fn stop(&self) {
        self.receive_task.abort();
        self.task.abort()
    }
}

pub struct TcpServer{
    listener: Arc<TcpListener>,
    receive_task:JoinHandle<()>,
    task:JoinHandle<()>
}

impl TcpServer{
    pub(crate) async fn bind(bind_at: String, task:JoinHandle<()>) -> crate::Result<Self>
    {

        let listener = Arc::new(TcpListener::bind(bind_at.clone()).await?);
        println!("TCP 服务器已启动，监听端口 {}...",bind_at);
        let lister_clone=listener.clone();
        let receive_task = tokio::task::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                // 为每个连接创建异步任务处理（不阻塞主线程）
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("处理连接出错: {}", e);
                    }
                });
            }
        });
        Ok(TcpServer{listener:lister_clone,receive_task,task})
    }
}
async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("新连接: {}", peer_addr);

    // 循环处理多个请求，直到客户端断开
    loop {
        let mut head_buf = [0u8; 4];
        stream.read_exact(&mut  head_buf).await?;
        let mut len_buf = [0u8; 2];
        stream.read_exact(&mut len_buf).await?; // 读长度
        let len = u16::from_be_bytes(len_buf) as usize;
        let mut data = vec![0u8; len-6];
        stream.read_exact(&mut data).await?; // 读消息体
        let mut vec_data = head_buf.to_vec();
        vec_data.extend(len_buf.to_vec());
        vec_data.extend(data.clone());
        process(vec_data).await;
        // println!("来自 {} 的数据: {:?}", peer_addr, vec_data);
    }
    Ok(())
}
impl Server for TcpServer{

    fn stop(&self) {
        self.receive_task.abort();
        self.task.abort()
    }
}
