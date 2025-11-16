use std::{
error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr, sync::Arc,
};

use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;

const DEFAULT_PORT: u16 = 50692;
const DEFAULT_MULTICAST: &str = "239.255.42.98";

enum Messages {
    SearchingForRooms,
    RoomHost,
    JoinRoom,
    LeaveRoom,
}

pub struct P2PClient {
    socket: Arc<UdpSocket>,
    multicast_addr: SocketAddrV4,
    is_host: bool,
    host: Option<SocketAddr>,
    name: String,
    is_connected: bool,
}

impl P2PClient {
    pub fn new(name: String) -> Self {
        let multicast_addr = SocketAddrV4::new(Ipv4Addr::from_str(DEFAULT_MULTICAST).unwrap(), DEFAULT_PORT);
        let std_socket = Self::bind_multicast(&multicast_addr).unwrap();
        let tokio_socket = UdpSocket::from_std(std_socket).unwrap();

        P2PClient {
            socket: Arc::from(tokio_socket),
            multicast_addr,
            name,
            is_host: false,
            host: None,
            is_connected: false,
        }
    }

    fn bind_multicast(
        multi_addr: &SocketAddrV4,
    ) -> Result<std::net::UdpSocket, Box<dyn Error>> {
        assert!(multi_addr.ip().is_multicast(), "Must be multcast address");

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_reuse_address(true)?;
        socket.set_nonblocking(true)?;
        socket.bind(&socket2::SockAddr::from(addr))?;
        socket.set_multicast_loop_v4(true)?;
        socket.join_multicast_v4(multi_addr.ip(), addr.ip())?;

        Ok(socket.into())
    }

    pub async fn host_loop(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.is_host = true;
        self.host = None;

        loop {
            let mut buffer = vec![0u8; 4096];
            let (n, sender) = self.socket.recv_from(&mut buffer[..]).await?;
            if n == 0 {
                break;
            }

            let received = String::from_utf8(buffer[..n].to_vec())?;
            if received.contains("Searching") {
                println!("Responding to room search from {:?}", sender);
                let response = format!("RoomHost:{}", self.name);
                self.send_multicast(response.as_bytes()).await?;
            }
            println!("Received: {:?}", received);
        }

        Ok(())

    }

    pub async fn client_loop(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.is_host = false;

        loop {
            let mut buffer = vec![0u8; 4096];
            let (n, sender) = self.socket.recv_from(&mut buffer[..]).await?;
            if n == 0 {
                break;
            }

            let received = String::from_utf8(buffer[..n].to_vec())?;
            if received.contains("RoomHost") {
                println!("Found host! {:?}", sender);
                self.host = Some(sender);
            }
            println!("Received: {:?}", received);
        }

        Ok(())
    }

    pub async fn search_rooms(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = format!("Searching:{}", self.name);
        self.socket.send_to(message.as_bytes(), &self.multicast_addr).await?;
        self.client_loop().await?;
        Ok(())
    }

    pub async fn send_multicast(&self, message: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.socket.send_to(message, &self.multicast_addr).await?;
        Ok(())
    }
}
