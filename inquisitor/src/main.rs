mod arp;

use anyhow::{anyhow, Result};
use arp::ArpClient;
use clap::Parser;
use pnet::{datalink::interfaces, ipnetwork::IpNetwork, packet::arp::ArpOperations, util::MacAddr};
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::arp::{Addrs, Message};

#[derive(Parser, Debug, Copy, Clone)]
pub struct Args {
    #[clap(value_name = "source ipv4")]
    pub sip: Ipv4Addr,
    #[clap(value_name = "source mac")]
    pub smac: MacAddr,
    #[clap(value_name = "target ipv4")]
    pub tip: Ipv4Addr,
    #[clap(value_name = "target mac")]
    pub tmac: MacAddr,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ctrlc = Arc::new(AtomicBool::new(false));
    let ctrlcclone = ctrlc.clone();
    ctrlc::set_handler(move || {
        ctrlcclone.store(true, Ordering::SeqCst);
    })?;

    let interfaces = interfaces();
    let iface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .ok_or(anyhow!("failed to find network interface"))?;
    println!("using interface: {}", iface.name);
    let client = ArpClient::new(iface)?;
    println!(
        "spoofing as {} ({}) to {} ({})",
        args.sip, args.smac, args.tip, args.smac
    );

    Ok(())
}
