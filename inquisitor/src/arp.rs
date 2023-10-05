use std::net::Ipv4Addr;

use anyhow::Result;
use pnet::{
    datalink::{self, Channel, DataLinkReceiver, DataLinkSender, NetworkInterface},
    packet::arp::ArpOperation,
    util::MacAddr,
};

pub struct ArpClient {
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,
}

impl ArpClient {
    pub fn new(iface: &NetworkInterface) -> Result<Self> {
        match datalink::channel(iface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => Ok(Self { tx, rx }),
            Ok(_) => Err(anyhow::anyhow!("unsupported datalink channel type")),
            Err(e) => Err(anyhow::anyhow!("failed to create datalink channel: {}", e)),
        }
    }
}

pub struct Message {
    pub source_ip: Ipv4Addr,
    pub source_mac: MacAddr,
    pub target_ip: Ipv4Addr,
    pub target_mac: MacAddr,
    pub operation: ArpOperation,
}

impl Message {
    pub fn new(
        source: (Ipv4Addr, MacAddr),
        target: (Ipv4Addr, MacAddr),
        operation: ArpOperation,
    ) -> Self {
        Self {
            source_ip: source.0,
            source_mac: source.1,
            target_ip: target.0,
            target_mac: target.1,
            operation,
        }
    }
}
