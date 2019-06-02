use super::*;

use std::convert::From;
use std::mem;

#[derive(Debug)]
pub struct RoutingTableData {
    pub dst: IpAddr,
    pub gateway: IpAddr,
    pub interface_index: i32,
    pub interface_label: String,
    pub mask: u8,
}

impl RoutingTableData {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl From<*const nlmsghdr> for RoutingTableData {
    fn from(nlp: *const nlmsghdr) -> Self {
        let mut rt = RoutingTableData::new();

        let rtp = nlmsg_data_rtmsg(nlp);
        let mut atp = rtm_rta(rtp);
        let mut atlen = rtm_payload(nlp);

        rt.mask = unsafe { (*rtp).rtm_dst_len };
        let ip_family = unsafe { (*rtp).rtm_family };

        while rta_ok(atp, atlen) {
            use RtAttrType::*;
            match num::FromPrimitive::from_u16(unsafe { (*atp).rta_type }) {
                Some(RTA_DST) => rt.dst = get_ip(ip_family, atp),
                Some(RTA_OIF) => rt.interface_index = unsafe { *(rta_data(atp) as *const i32) },
                Some(RTA_GATEWAY) => rt.gateway = get_ip(ip_family, atp),
                _ => {}
            }

            atp = rta_next(atp, &mut atlen);
        }

        rt.interface_label = get_ifname(rt.interface_index);

        rt
    }
}
