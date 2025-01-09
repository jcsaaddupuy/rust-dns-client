use message::header::MessageHeader;
use message::message::Message;
use nom::{AsBytes, HexDisplay};
use rr::{record_class::Class, record_type::RecordType};
use tokio::net::UdpSocket;

mod message;
mod rr;
use bitvec::prelude::*;
use log::{error, info};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    colog::init();

    // let resolver = "8.8.8.8:53";
    let resolver = "127.0.0.1:1053";

    let query_id = rand::thread_rng().gen::<u16>();
    // let query_id = 1;
    let message: Message = Message::new(query_id, "google.com", RecordType::A, Class::IN)
        .expect("Could not build message");
    let local_addr = "0.0.0.0:0";
    let socket = UdpSocket::bind(local_addr)
        .await
        .expect("couldn't bind to a local address");

    socket
        .connect(resolver)
        .await
        .expect("couldn't connect to the DNS resolver");

    // Send the DNS resolver the message
    let body: Vec<u8> = message.as_vec();

    info!("bytes to send : {}", hex::encode(body.as_bytes()));

    // return Ok(());
    let bytes_sent = socket.send(&body).await.expect("couldn't send data");
    if bytes_sent != body.len() {
        panic!("Only {bytes_sent} bytes, message was probably truncated");
    }

    let mut response_buf = vec![0; message::message::MAX_UDP_BYTES];
    match socket.recv(&mut response_buf).await {
        Ok(received) => {
            let value = response_buf[..received].to_vec();

            let result = MessageHeader::try_from(value);

            match result {
                Ok(header) => {
                    info!("header : {:?}", header)
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
