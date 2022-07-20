#![allow(dead_code)]

use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

use clap::Parser;

struct Payload {
    client_version: ClientVersion,
    client_random: ClientRandom,
    session_id: SessionID,
    cipher_suites: Vec<CipherSuite>,
    compression_methods: Vec<CompressionMethod>,
    extentions: Vec<Extension>,
}

enum ClientVersion {}
struct ClientRandom;
struct SessionID;
enum CipherSuite {}
enum CompressionMethod {}
enum Extension {}

#[derive(Parser, Debug)]
struct Args {
    // IPv4 address
    #[clap(short, long, value_parser, name = "ipv4-address")]
    ipv4_address: Ipv4Addr,
    #[clap(short, long, value_parser)]
    port: u16,
}

// work flow:
// 1. connect to TLS server with TCP
// 2. send TLS 1.3 Client Hello message
// 3. receive decode Server Hello message
fn main() {
    let args = Args::parse();

    let address = SocketAddrV4::new(args.ipv4_address, args.port);
    let mut _stream = TcpStream::connect(address).unwrap();
}
