use super::*;

use std::convert::From;
use std::mem;

#[derive(Debug)]
pub struct NeighData {
    pub interface_index: i32,
    pub interface_label: String,
    pub cache_ip_dst_addr: IpAddr,
    pub cache_mac_addr: MacAddr,
    pub cache_info: nda_cacheinfo,
    pub incomplete: bool,
    pub reachable: bool,
    pub stale: bool,
    pub delay: bool,
    pub probe: bool,
    pub failed: bool,
    pub noarp: bool,
    pub permanent: bool,
}

impl NeighData {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl From<*const nlmsghdr> for NeighData {
    fn from(nlp: *const nlmsghdr) -> Self {
        let mut neigh_data = NeighData::new();

        let nda = nlmsg_data_ndmsg(nlp);
        let mut atp = nda_rta(nda);
        let mut atlen = nda_payload(nlp);

        neigh_data.interface_index = unsafe { (*nda).ndm_ifindex };
        neigh_data.interface_label = get_ifname(neigh_data.interface_index);

        {
            use NeighborCacheEntryStates::*;
            let s = unsafe { (*nda).ndm_state };

            neigh_data.incomplete = s & NUD_INCOMPLETE > 0;
            neigh_data.reachable = s & NUD_REACHABLE > 0;
            neigh_data.stale = s & NUD_STALE > 0;
            neigh_data.delay = s & NUD_DELAY > 0;
            neigh_data.probe = s & NUD_PROBE > 0;
            neigh_data.failed = s & NUD_FAILED > 0;
            neigh_data.noarp = s & NUD_NOARP > 0;
            neigh_data.permanent = s & NUD_PERMANENT > 0;
        }

        let ndm_family = unsafe { (*nda).ndm_family };

        while rta_ok(atp, atlen) {
            use NdaAttrType::*;

            match num::FromPrimitive::from_u16(unsafe { (*atp).rta_type }) {
                Some(NDA_DST) => neigh_data.cache_ip_dst_addr = get_ip(ndm_family, atp),
                Some(NDA_LLADDR) => {
                    neigh_data.cache_mac_addr = unsafe { *(rta_data(atp) as *const MacAddr) }
                }
                Some(NDA_CACHEINFO) => {
                    neigh_data.cache_info = unsafe { *(rta_data(atp) as *const nda_cacheinfo) }
                }
                _ => {}
            }

            atp = rta_next(atp, &mut atlen);
        }

        neigh_data
    }
}
