use std::io;
use std::io::Write;
use std::error::Error;
use std::net::{SocketAddr, SocketAddrV4};

use tokio::net::UdpSocket;
use socket2::{Domain, Type, Protocol, Socket};

pub fn bind_multicast(
    addr: &SocketAddrV4,
    multi_addr: &SocketAddrV4,
) -> Result<std::net::UdpSocket, Box<dyn Error>> {
    assert!(multi_addr.ip().is_multicast(), "Must be multcast address");

    let socket = Socket::new(
        Domain::IPV4,
        Type::DGRAM,
        Some(Protocol::UDP),
    )?;

    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&socket2::SockAddr::from(*addr))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(
        multi_addr.ip(),
        addr.ip(),
    )?;

    Ok(socket.into())
}


/// Receive bytes from UPD socket and write to stdout until EOF.
pub async fn receive(rx: UdpSocket) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut buffer = vec![0u8; 4096];
    let mut stdout = io::stdout();

    loop {
        let n = rx.recv(&mut buffer[..]).await?;
        if n == 0 {
            break;
        }
        println!("RECIEVED! size: {}", n);
        stdout.write_all(&mut buffer[..n])?;
    }

    Ok(())
}

/// Transmit bytes from stdin until EOF, Ctrl+D on linux or Ctrl+Z on windows.
pub async fn transmit(
    tx: UdpSocket,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    for i in 1..10 {
        let message = format!("hellow {}", i);
        println!("sending message: {}", message);
        tx.send_to(&message.into_bytes(), &addr).await?;
    }

    Ok(())
}
