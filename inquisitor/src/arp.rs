use std::net::Ipv4Addr;

use anyhow::{anyhow, Result};
use pnet::{
    datalink::{self, Channel, DataLinkReceiver, DataLinkSender, NetworkInterface},
    ipnetwork::IpNetwork,
    packet::{
        arp::{
            ArpHardwareTypes, ArpOperation,
            ArpOperations::{self, Reply},
            ArpPacket, MutableArpPacket,
        },
        ethernet::{EtherTypes, MutableEthernetPacket},
    },
    util::MacAddr,
};

pub struct Addrs {
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
}
impl Addrs {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        Self { ip, mac }
    }
}

pub struct ArpClient {
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,
    ip: Ipv4Addr,
    mac: MacAddr,
}

impl ArpClient {
    pub fn new(iface: &NetworkInterface) -> Result<Self> {
        let ip = iface
            .ips
            .iter()
            .find_map(|ip| match ip {
                IpNetwork::V4(v4) => Some(v4),
                _ => None,
            })
            .ok_or(anyhow!("didn't find a ipv4 address on interface"))?
            .ip();
        let mac = iface
            .mac
            .ok_or(anyhow!("didn't find a mac address on interface"))?;
        match datalink::channel(iface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => Ok(Self { tx, rx, ip, mac }),
            Ok(_) => Err(anyhow!("unsupported datalink channel type")),
            Err(e) => Err(anyhow!("failed to create datalink channel: {}", e)),
        }
    }

    pub fn spoof(
        &mut self,
        source: (Ipv4Addr, MacAddr),
        target: (Ipv4Addr, MacAddr),
    ) -> Result<()> {
        let buffer = &mut [0u8; 42];
        let mut packet = MutableEthernetPacket::new(buffer)
            .ok_or(anyhow!("MutableEthernetPacket returned None"))?;
        packet.set_source(self.mac);
        packet.set_destination(target.1);
        packet.set_ethertype(EtherTypes::Arp);
        let msg: ArpPacket = {
            let mut arp_buf = [0u8; 28];
            let mut arp_pkt = MutableArpPacket::new(arp_buf.as_mut_slice())
                .ok_or(anyhow!("MutableArpPacket returned None"))?;
            arp_pkt.set_hardware_type(ArpHardwareTypes::Ethernet);
            arp_pkt.set_protocol_type(EtherTypes::Ipv4);
            // arp_pkt.set_hw_addr_len(6);
            arp_pkt.set_sender_hw_addr(self.mac);
            arp_pkt.set_target_hw_addr(source.1);
            // arp_pkt.set_proto_addr_len(4);
            arp_pkt.set_sender_proto_addr(target.0);
            arp_pkt.set_target_proto_addr(source.0);
            arp_pkt.set_operation(ArpOperations::Reply);
            arp_pkt.consume_to_immutable()
        };
        Ok(())
    }

    pub fn recv(&mut self) -> Result<&[u8]> {
        Ok(self.rx.next()?)
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
