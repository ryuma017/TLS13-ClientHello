#![allow(dead_code, unreachable_code)]

mod client_hello;

use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

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
// 3. receive decode Server Hello message
fn main() {
    let args = Args::parse();

    let address = SocketAddrV4::new(args.ipv4_address, args.port);
    let mut _stream = TcpStream::connect(address).unwrap();
}
