use super::*;

use std::convert::From;
use std::mem;

#[derive(Debug)]
pub struct IpData {
    pub local_addr: IpAddr,
    pub interface_addr: IpAddr,
    pub interface_index: i32,
    pub interface_label: String,
    pub mask: u8,
}

impl IpData {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl From<*const nlmsghdr> for IpData {
    fn from(nlp: *const nlmsghdr) -> Self {
        let mut ip_data = IpData::new();

        let ifa = nlmsg_data_ifaddrmsg(nlp);
        let mut atp = ifa_rta(ifa);
        let mut atlen = ifa_payload(nlp);

        let ifa_family = unsafe { (*ifa).ifa_family };
        ip_data.mask = unsafe { (*ifa).ifa_prefixlen };
        ip_data.interface_index = unsafe { (*ifa).ifa_index } as i32;
        ip_data.interface_label = get_ifname(ip_data.interface_index);

        while rta_ok(atp, atlen) {
            use IfaAttrType::*;

            match num::FromPrimitive::from_u16(unsafe { (*atp).rta_type }) {
                Some(IFA_ADDRESS) => ip_data.interface_addr = get_ip(ifa_family, atp),
                Some(IFA_LOCAL) => ip_data.local_addr = get_ip(ifa_family, atp),
                _ => {}
            }

            atp = rta_next(atp, &mut atlen);
        }

        ip_data
    }
}
