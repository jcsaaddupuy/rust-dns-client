use bitvec::{order::Msb0, slice::BitSlice};
use tokio::net::UdpSocket;
mod dns;
mod message;
use log::{info, warn};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    colog::init();

    let record: dns::RecordType= 1.try_into().unwrap();
    let slice: &BitSlice<u16, Msb0> = record.into();
    info!("{:?}", slice);

    // let local_addr = "0.0.0.0:0";
    // let socket = UdpSocket::bind(local_addr)
    //     .await
    //     .expect("couldn't bind to a local address");
    //socket.set_read_timeout(Some(Duration::from_secs(5)))?;
}
