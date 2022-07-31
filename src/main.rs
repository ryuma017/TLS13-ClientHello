mod client_hello;

use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

use client_hello::{ClientHello, HandshakeMessage, TLSPlaintext};

#[derive(Parser, Debug)]
struct Args {
    // IPv4 address
    #[clap(short, long, value_parser, name = "ipv4-address")]
    ipv4_address: Ipv4Addr,
    #[clap(short, long, value_parser)]
    port: u16,
}

// [Workflow]
// 1. connect to TLS server with TCP
// 2. send TLS 1.3 Client Hello message
// 3. receive Server Hello message
fn communicate_with_server(address: SocketAddrV4, payload: &[u8]) {
    let mut stream = TcpStream::connect(address).unwrap();
    todo!()
}

fn main() {
    let args = Args::parse();

    let address = SocketAddrV4::new(args.ipv4_address, args.port);
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
