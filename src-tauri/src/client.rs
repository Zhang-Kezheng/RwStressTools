use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
#[async_trait]
pub trait Client {
    async fn write(&mut self, data: &[u8]) -> crate::Result<()>;
    async fn stop(&mut self) -> crate::Result<()>;
}

pub struct TcpClient {
    tcp_stream: TcpStream,
}
impl TcpClient {
    pub(crate) async fn new(target: String) -> crate::Result<Self> {
        let tcp_stream = TcpStream::connect(target).await?;
        Ok(TcpClient { tcp_stream })
    }
}
#[async_trait]
impl Client for TcpClient {
    async fn write(&mut self, data: &[u8]) -> crate::Result<()> {
        self.tcp_stream.write_all(data).await?;
        self.tcp_stream.flush().await?;
        Ok(())
    }

    async fn stop(&mut self) -> crate::Result<()> {
        self.tcp_stream.shutdown().await?;
        Ok(())
    }
}

pub struct UdpClient {
    udp_socket: UdpSocket,
    target: String,
}
impl UdpClient {
    pub(crate) async fn new(target: String) -> crate::Result<Self> {
        let udp_socket = UdpSocket::bind("127.0.0.1:0").await?;
        Ok(UdpClient { udp_socket, target })
    }
}

#[async_trait]
impl Client for UdpClient {
    async fn write(&mut self, data: &[u8]) -> crate::Result<()> {
        self.udp_socket.send_to(data, self.target.as_str()).await?;
        Ok(())
    }

    async fn stop(&mut self) -> crate::Result<()> {
        Ok(())
    }
}
