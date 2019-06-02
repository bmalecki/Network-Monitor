use crate::mac_addr::MacAddr;
use crate::rtnetlink::*;
use libc::*;
use std::convert::From;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

mod ip_data;
mod link_data;
mod neigh_data;
mod routing_table_data;

pub use self::ip_data::*;
pub use self::link_data::*;
pub use self::neigh_data::*;
pub use self::routing_table_data::*;

#[derive(Debug)]
pub struct RtMsg {
    pub event: RtEvent,
    pub time: std::time::SystemTime,
    pub data: RtData,
}

impl RtMsg {
    fn new(event: RtEvent, data: RtData) -> Self {
        RtMsg {
            event,
            time: std::time::SystemTime::now(),
            data,
        }
    }

    pub fn on_new(data: RtData) -> Option<Self> {
        Some(Self::new(RtEvent::New, data))
    }

    pub fn on_delete(data: RtData) -> Option<Self> {
        Some(Self::new(RtEvent::Delete, data))
    }

    pub fn on_unsuported(msg: String) -> Option<Self> {
        Some(Self::new(RtEvent::Unsuported, RtData::Unsuported(msg)))
    }
}

#[derive(Debug)]
pub enum RtEvent {
    New,
    Delete,
    Unsuported,
}

#[derive(Debug)]
pub enum RtData {
    Routing(RoutingTableData),
    Ip(IpData),
    Link(LinkData),
    Neigh(NeighData),
    Unsuported(String),
}

fn get_ip(ip_family: u8, atp: *const rtattr) -> IpAddr {
    match ip_family {
        2 => IpAddr::V4(unsafe { *(rta_data(atp) as *const Ipv4Addr) }),
        10 => IpAddr::V6(unsafe { *(rta_data(atp) as *const Ipv6Addr) }),
        n => panic!("Not know ip family number: {}", n),
    }
}

fn get_ifname(interface_index: i32) -> String {
    match indextoname(interface_index) {
        Some(label) => label,
        None => String::from(""),
    }
}
