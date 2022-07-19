use std::net::Ipv4Addr;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // IPv4 address
    #[clap(short, long, value_parser, name = "ipv4-address")]
    ipv4_address: Ipv4Addr,
    #[clap(short, long, value_parser)]
    port: u32
}

fn main() {
    let args = Args::parse();
    let host = args.ipv4_address;
    let port = args.port;

    println!("{host}:{port}");
}
