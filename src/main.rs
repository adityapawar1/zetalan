use p2p::P2PClient;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};
use tokio::net::UdpSocket;

mod app;
mod p2p;
mod zetamac;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // let mut client = P2PClient::new("Player".to_string());
    // client.search_rooms().await?;
    let mut host = P2PClient::new("Host".to_string());
    host.host_loop().await?;
    // receive(socket).await?;

    Ok(())

    // let mut terminal = ratatui::init();
    // let app_result = App::default().run(&mut terminal);
    // ratatui::restore();
    // app_result
}
