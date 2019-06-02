pub mod rtmsg;

use crate::rtnetlink::*;
use libc::*;
pub use rtmsg::*;
use std::convert::From;
use std::mem;
use std::process;

pub struct RtSocket {
    fd: i32,
}

impl RtSocket {
    pub fn new() -> Self {
        let fd = unsafe { socket(PF_NETLINK, SOCK_RAW, NETLINK_ROUTE) };

        if fd < 0 {
            panic!("Can not open file descriptor")
        }

        let mut snl: sockaddr_nl = unsafe { mem::zeroed() };
        snl.nl_family = AF_NETLINK as u16;
        snl.nl_pid = process::id();

        use MulticastGroup::*;

        // snl.nl_groups = RTMGRP_NOTIFY | RTMGRP_NEIGH;

        snl.nl_groups = RTMGRP_IPV4_IFADDR
            | RTMGRP_IPV6_IFADDR
            | RTMGRP_IPV4_ROUTE
            | RTMGRP_IPV6_ROUTE
            | RTMGRP_NOTIFY
            | RTMGRP_NEIGH
            | RTMGRP_LINK;

        // snl.nl_groups = 0xFFF;

        unsafe {
            bind(
                fd,
                mem::transmute(&snl),
                mem::size_of::<sockaddr_nl>() as u32,
            );
        }

        RtSocket { fd }
    }

    pub fn receive<T>(&self, mut callback: T)
    where
        T: FnMut(Option<RtMsg>),
    {
        let mut buf: Vec<usize> = vec![0; 8192];

        let mut ptr = &mut buf[0] as *mut usize;
        let mut nllen: i32 = 0;

        loop {
            unsafe {
                let rclen = recv(self.fd, mem::transmute(ptr), buf.len() - nllen as usize, 0);
                let nlp: *const nlmsghdr = mem::transmute(ptr);
                nllen += rclen as i32;
                ptr = ptr.offset(nllen as isize);

                if (*nlp).nlmsg_type != NLMSG_DONE as u16 {
                    break;
                }
            }
        }

        let ptr = &buf[0] as *const usize;
        let mut nlp: *const nlmsghdr = unsafe { mem::transmute(ptr) };

        while nlmsg_ok(nlp, nllen) {
            use MsgType::*;
            use RtData::*;

            let msg = match num::FromPrimitive::from_u16(unsafe { (*nlp).nlmsg_type }) {
                Some(RTM_NEWROUTE) => RtMsg::on_new(Routing(RoutingTableData::from(nlp))),
                Some(RTM_DELROUTE) => RtMsg::on_delete(Routing(RoutingTableData::from(nlp))),
                Some(RTM_NEWADDR) => RtMsg::on_new(Ip(IpData::from(nlp))),
                Some(RTM_DELADDR) => RtMsg::on_delete(Ip(IpData::from(nlp))),
                Some(RTM_NEWLINK) => RtMsg::on_new(Link(LinkData::from(nlp))),
                Some(RTM_DELLINK) => RtMsg::on_delete(Link(LinkData::from(nlp))),
                Some(RTM_NEWNEIGH) => RtMsg::on_new(Neigh(NeighData::from(nlp))),
                Some(RTM_DELNEIGH) => RtMsg::on_delete(Neigh(NeighData::from(nlp))),
                Some(msg) => RtMsg::on_unsuported(format!("Unsuported MsgType: {:?}", msg)),
                None => RtMsg::on_unsuported(format!("Unknow MsgType")),
            };

            callback(msg);
            nlp = nlmsg_next(nlp, &mut nllen);
        }
    }
}

impl Drop for RtSocket {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

impl Iterator for RtSocket {
    type Item = RtMsg;
    
    fn next(&mut self) -> Option<RtMsg> {
        let mut msg: Option<RtMsg> = None;

        self.receive(|m| {
            msg = m;
        });

        msg
    }
}