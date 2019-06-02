use super::*;

use std::convert::From;
use std::mem;

#[derive(Debug)]
pub struct LinkData {
    pub interface_index: i32,
    pub interface_label: String,
    pub mac_addr: MacAddr,
    pub promiscuous: bool,
    pub running: bool,
    pub up: bool,
    pub broadcast: bool,
    pub multicast: bool,
    pub mtu: u32,
}

impl LinkData {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl From<*const nlmsghdr> for LinkData {
    fn from(nlp: *const nlmsghdr) -> Self {
        let mut link_data = LinkData::new();

        let ifi = nlmsg_data_ifinfomsg(nlp);
        let mut atp = ifla_rta(ifi);
        let mut atlen = ifla_payload(nlp);

        link_data.interface_index = unsafe { (*ifi).ifi_index };
        link_data.interface_label = get_ifname(link_data.interface_index);

        {
            let f = unsafe { (*ifi).ifi_flags };
            use IffDeviceFlags::*;

            link_data.up = f & IFF_UP > 0;
            link_data.running = f & IFF_RUNNING > 0;
            link_data.promiscuous = f & IFF_PROMISC > 0;
            link_data.broadcast = f & IFF_BROADCAST > 0;
            link_data.multicast = f & IFF_MULTICAST > 0;
        }

        while rta_ok(atp, atlen) {
            use IflaAttrType::*;

            match num::FromPrimitive::from_u16(unsafe { (*atp).rta_type }) {
                Some(IFLA_ADDRESS) => {
                    link_data.mac_addr = unsafe { *(rta_data(atp) as *const MacAddr) }
                }
                Some(IFLA_MTU) => link_data.mtu = unsafe { *(rta_data(atp) as *const u32) },
                _ => {}
            }

            atp = rta_next(atp, &mut atlen);
        }

        link_data
    }
}