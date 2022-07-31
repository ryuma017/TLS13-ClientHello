mod client_hello;

use std::{
    io::{BufReader, Write, Read},
    net::{IpAddr, SocketAddr, TcpStream},
    time::Duration,
};

use clap::Parser;
use client_hello::{ClientHello, HandshakeMessage, TLSPlaintext};

#[derive(Parser, Debug)]
struct Args {
    // IPv4 address
    #[clap(short, long, value_parser, name = "ipv4-address")]
    address: IpAddr,
    #[clap(short, long, value_parser)]
    port: u16,
}

// [Workflow]
// 1. connect to TLS server with TCP
// 2. send TLS 1.3 Client Hello message
// 3. receive Server Hello message
fn communicate_with_server(address: SocketAddr, payload: &[u8]) {
    let mut stream =
        TcpStream::connect_timeout(&address, Duration::from_secs(1)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    stream.write_all(payload).unwrap();

    let mut reader = BufReader::new(&stream);
    let mut buffer = [0; 1024];
    let length = reader.read(&mut buffer).unwrap();
    // let mut buffer = Vec::new();
    // reader.read(&mut buffer).unwrap();

    println!("{:?}", &buffer[..length]);
}

fn main() {
    let args = Args::parse();

    let address = SocketAddr::new(args.address, args.port);
    let payload = {
        let client_hello_payload = ClientHello::default_v1_3();
        let handshake = HandshakeMessage::new_client_hello(client_hello_payload);
        let plaintext = TLSPlaintext::new_handshake(handshake);
        let mut bytes: Vec<u8> = Vec::new();
        plaintext.encode(&mut bytes);
        bytes
    };

    communicate_with_server(address, &payload);
}
