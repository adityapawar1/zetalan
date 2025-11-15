use std::{error::Error, net::{Ipv4Addr, SocketAddrV4}, str::FromStr};
use p2p::*;
use tokio::net::UdpSocket;

mod zetamac;
mod app;
mod p2p;

const DEFAULT_PORT: u16 = 50692;
const DEFAULT_MULTICAST: &str = "239.255.42.98";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let multi_addr = SocketAddrV4::new(Ipv4Addr::from_str(DEFAULT_MULTICAST).unwrap(), DEFAULT_PORT);
    let socket: UdpSocket = UdpSocket::from_std(bind_multicast(&addr, &multi_addr).unwrap()).unwrap();

    transmit(socket, multi_addr.into()).await?;
    // receive(socket).await?;

    Ok(())

    // let mut terminal = ratatui::init();
    // let app_result = App::default().run(&mut terminal);
    // ratatui::restore();
    // app_result
}
