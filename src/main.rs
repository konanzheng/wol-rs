use clap::Parser;
use std::net::IpAddr;
use wol::{send_wol, MacAddr};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Destination MAC address
    #[clap(name = "MAC_ADDR")]
    mac_addr: String,

    /// Broadcast IP address
    #[clap(short = 'c', long, default_value = "255.255.255.255")]
    bcast_addr: String,

    /// Bind IP address
    #[clap(short = 'b', long, default_value = "0.0.0.0")]
    bind_addr: String,
}

fn main() {
    let args = Args::parse();
    let mac_addr: MacAddr = args.mac_addr.parse().unwrap();
    let bcast_addr: IpAddr = args.bcast_addr.parse().unwrap();
    let bind_addr: IpAddr = args.bind_addr.parse().unwrap();
    send_wol(mac_addr, Some(bcast_addr), Some(bind_addr)).unwrap();
}
