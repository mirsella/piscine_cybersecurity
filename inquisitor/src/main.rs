mod arp;

use anyhow::{anyhow, Result};
use arp::ArpClient;
use clap::Parser;
use pnet::{datalink::interfaces, util::MacAddr};
use std::{
    net::Ipv4Addr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Parser, Debug, Copy, Clone)]
struct Args {
    #[clap(value_name = "source ipv4")]
    sip: Ipv4Addr,
    #[clap(value_name = "source mac")]
    smac: MacAddr,
    #[clap(value_name = "target ipv4")]
    tip: Ipv4Addr,
    #[clap(value_name = "target mac")]
    tmac: MacAddr,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ctrlc = Arc::new(AtomicBool::new(false));
    let ctrlcclone = ctrlc.clone();
    ctrlc::set_handler(move || {
        ctrlcclone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let interfaces = interfaces();
    let iface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .ok_or(anyhow!("failed to find network interface"))?;
    let client = ArpClient::new(iface)?;
    println!(
        "spoofing as {} ({}) to {} ({})",
        args.sip, args.smac, args.tip, args.smac
    );
    Ok(())
}
