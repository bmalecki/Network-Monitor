use libc::*;
use std::ffi::CStr;
use std::mem;

mod c {
	use super::*;

	extern "C" {
		pub fn if_indextoname(ifindex: c_int, ifname: *mut c_char) -> *mut c_char;
		pub fn nlmsg_ok(nlp: *const nlmsghdr, len: c_int) -> bool;
		pub fn nlmsg_next(nlp: *const nlmsghdr, len: *mut c_int) -> *const nlmsghdr;
		pub fn nlmsg_data(nlp: *const nlmsghdr) -> *const c_void;
		pub fn rtm_rta(rta: *const rtmsg) -> *const rtattr;
		pub fn ifa_rta(rta: *const ifaddrmsg) -> *const rtattr;
		pub fn ifla_rta(rta: *const ifinfomsg) -> *const rtattr;
		pub fn nda_rta(rta: *const ndmsg) -> *const rtattr;
		pub fn rtm_payload(nlp: *const nlmsghdr) -> c_int;
		pub fn ifa_payload(nlp: *const nlmsghdr) -> c_int;
		pub fn ifla_payload(nlp: *const nlmsghdr) -> c_int;
		pub fn nda_payload(nlp: *const nlmsghdr) -> c_int;
		pub fn rta_ok(atp: *const rtattr, len: c_int) -> bool;
		pub fn rta_next(atp: *const rtattr, len: *mut c_int) -> *const rtattr;
		pub fn rta_data(atp: *const rtattr) -> *const c_void;
	}
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum MulticastGroup {
	RTMGRP_LINK = 1,
	RTMGRP_NOTIFY = 2,
	RTMGRP_NEIGH = 4,
	RTMGRP_TC = 8,
	RTMGRP_IPV4_IFADDR = 0x10,
	RTMGRP_IPV4_MROUTE = 0x20,
	RTMGRP_IPV4_ROUTE = 0x40,
	RTMGRP_IPV4_RULE = 0x80,
	RTMGRP_IPV6_IFADDR = 0x100,
	RTMGRP_IPV6_MROUTE = 0x200,
	RTMGRP_IPV6_ROUTE = 0x400,
	RTMGRP_IPV6_IFINFO = 0x800,
}

impl std::ops::BitOr for MulticastGroup {
	type Output = u32;
	fn bitor(self, rhs: MulticastGroup) -> u32 {
		self as u32 | rhs as u32
	}
}

impl std::ops::BitOr<MulticastGroup> for u32 {
	type Output = u32;
	fn bitor(self, rhs: MulticastGroup) -> u32 {
		self | rhs as u32
	}
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum IffDeviceFlags {
	IFF_UP = 0x1,           /* Interface is up.  */
	IFF_BROADCAST = 0x2,    /* Broadcast address valid.  */
	IFF_DEBUG = 0x4,        /* Turn on debugging.  */
	IFF_LOOPBACK = 0x8,     /* Is a loopback net.  */
	IFF_POINTOPOINT = 0x10, /* Interface is point-to-point link.  */
	IFF_NOTRAILERS = 0x20,  /* Avoid use of trailers.  */
	IFF_RUNNING = 0x40,     /* Resources allocated.  */
	IFF_NOARP = 0x80,       /* No address resolution protocol.  */
	IFF_PROMISC = 0x100,    /* Receive all packets.  */
	/* Not supported */
	IFF_ALLMULTI = 0x200,   /* Receive all multicast packets.  */
	IFF_MASTER = 0x400,     /* Master of a load balancer.  */
	IFF_SLAVE = 0x800,      /* Slave of a load balancer.  */
	IFF_MULTICAST = 0x1000, /* Supports multicast.  */
	IFF_PORTSEL = 0x2000,   /* Can set media type.  */
	IFF_AUTOMEDIA = 0x4000, /* Auto media select active.  */
	IFF_DYNAMIC = 0x8000,   /* Dialup device with changing addresses.  */
}

impl std::ops::BitOr for IffDeviceFlags {
	type Output = u32;
	fn bitor(self, rhs: IffDeviceFlags) -> u32 {
		self as u32 | rhs as u32
	}
}

impl std::ops::BitOr<IffDeviceFlags> for u32 {
	type Output = u32;
	fn bitor(self, rhs: IffDeviceFlags) -> u32 {
		self | rhs as u32
	}
}

impl std::ops::BitAnd for IffDeviceFlags {
	type Output = u32;
	fn bitand(self, rhs: IffDeviceFlags) -> u32 {
		self as u32 & rhs as u32
	}
}

impl std::ops::BitAnd<IffDeviceFlags> for u32 {
	type Output = u32;
	fn bitand(self, rhs: IffDeviceFlags) -> u32 {
		self & rhs as u32
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct rtmsg {
	pub rtm_family: u8,
	pub rtm_dst_len: u8,
	pub rtm_src_len: u8,
	pub rtm_tos: u8,
	pub rtm_table: u8,
	pub rtm_protocol: u8,
	pub rtm_scope: u8,
	pub rtm_type: u8,
	pub rtm_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ifaddrmsg {
	pub ifa_family: u8,    /* Address type */
	pub ifa_prefixlen: u8, /* Prefixlength of address */
	pub ifa_flags: u8,     /* Address flags */
	pub ifa_scope: u8,     /* Address scope */
	pub ifa_index: u32,    /* Interface index */
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ifinfomsg {
	pub ifi_family: u8,  /* AF_UNSPEC */
	pub ifi_type: u16,   /* Device type */
	pub ifi_index: i32,  /* Interface index */
	pub ifi_flags: u32,  /* Device flags  */
	pub ifi_change: u32, /* change mask */
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ndmsg {
	pub ndm_family: u8,
	ndm_pad1: u8,
	ndm_pad2: u16,
	pub ndm_ifindex: i32,
	pub ndm_state: u16,
	pub ndm_flags: u8,
	pub ndm_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct nda_cacheinfo {
	pub ndm_confirmed: u32,
	pub ndm_used: u32,
	pub ndm_updated: u32,
	pub ndm_refcnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtattr {
	pub rta_len: u16,
	pub rta_type: u16,
}

/* Types of messages */
#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum MsgType {
	RTM_NEWLINK = 16,
	RTM_DELLINK,
	RTM_GETLINK,
	RTM_SETLINK,
	RTM_NEWADDR = 20,
	RTM_DELADDR,
	RTM_GETADDR,
	RTM_NEWROUTE = 24,
	RTM_DELROUTE,
	RTM_GETROUTE,
	RTM_NEWNEIGH = 28,
	RTM_DELNEIGH,
	RTM_GETNEIGH,
	RTM_NEWRULE = 32,
	RTM_DELRULE,
	RTM_GETRULE,
	RTM_NEWQDISC = 36,
	RTM_DELQDISC,
	RTM_GETQDISC,
	RTM_NEWTCLASS = 40,
	RTM_DELTCLASS,
	RTM_GETTCLASS,
	RTM_NEWTFILTER = 44,
	RTM_DELTFILTER,
	RTM_GETTFILTER,
	RTM_NEWACTION = 48,
	RTM_DELACTION,
	RTM_GETACTION,
	RTM_NEWPREFIX = 52,
	RTM_GETMULTICAST = 58,
	RTM_GETANYCAST = 62,
	RTM_NEWNEIGHTBL = 64,
	RTM_GETNEIGHTBL = 66,
	RTM_SETNEIGHTBL,
	RTM_NEWNDUSEROPT = 68,
	RTM_NEWADDRLABEL = 72,
	RTM_DELADDRLABEL,
	RTM_GETADDRLABEL,
	RTM_GETDCB = 78,
	RTM_SETDCB,
	RTM_NEWNETCONF = 80,
	RTM_DELNETCONF,
	RTM_GETNETCONF = 82,
	RTM_NEWMDB = 84,
	RTM_DELMDB = 85,
	RTM_GETMDB = 86,
	RTM_NEWNSID = 88,
	RTM_DELNSID = 89,
	RTM_GETNSID = 90,
	RTM_NEWSTATS = 92,
	RTM_GETSTATS = 94,
	RTM_NEWCACHEREPORT = 96,
}

/* Routing message attributes */
#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum RtAttrType {
	RTA_UNSPEC,
	RTA_DST,
	RTA_SRC,
	RTA_IIF,
	RTA_OIF,
	RTA_GATEWAY,
	RTA_PRIORITY,
	RTA_PREFSRC,
	RTA_METRICS,
	RTA_MULTIPATH,
	RTA_PROTOINFO, /* no longer used */
	RTA_FLOW,
	RTA_CACHEINFO,
	RTA_SESSION, /* no longer used */
	RTA_MP_ALGO, /* no longer used */
	RTA_TABLE,
	RTA_MARK,
	RTA_MFC_STATS,
	RTA_VIA,
	RTA_NEWDST,
	RTA_PREF,
	RTA_ENCAP_TYPE,
	RTA_ENCAP,
	RTA_EXPIRES,
	RTA_PAD,
	RTA_UID,
	RTA_TTL_PROPAGATE,
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum IfaAttrType {
	IFA_UNSPEC,
	IFA_ADDRESS,
	IFA_LOCAL,
	IFA_LABEL,
	IFA_BROADCAST,
	IFA_ANYCAST,
	IFA_CACHEINFO,
	IFA_MULTICAST,
	IFA_FLAGS,
	IFA_RT_PRIORITY, /* u32, priority/metric for prefix route */
	IFA_TARGET_NETNSID,
	__IFA_MAX,
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum IflaAttrType {
	IFLA_UNSPEC,
	IFLA_ADDRESS,
	IFLA_BROADCAST,
	IFLA_IFNAME,
	IFLA_MTU,
	IFLA_LINK,
	IFLA_QDISC,
	IFLA_STATS,
	IFLA_COST,
	IFLA_PRIORITY,
	IFLA_MASTER,
	IFLA_WIRELESS, /* Wireless Extension event - see wireless.h */
	IFLA_PROTINFO, /* Protocol specific information for a link */
	IFLA_TXQLEN,
	IFLA_MAP,
	IFLA_WEIGHT,
	IFLA_OPERSTATE,
	IFLA_LINKMODE,
	IFLA_LINKINFO,
	IFLA_NET_NS_PID,
	IFLA_IFALIAS,
	IFLA_NUM_VF, /* Number of VFs if device is SR-IOV PF */
	IFLA_VFINFO_LIST,
	IFLA_STATS64,
	IFLA_VF_PORTS,
	IFLA_PORT_SELF,
	IFLA_AF_SPEC,
	IFLA_GROUP, /* Group the device belongs to */
	IFLA_NET_NS_FD,
	IFLA_EXT_MASK,    /* Extended info mask, VFs, etc */
	IFLA_PROMISCUITY, /* Promiscuity count: > 0 means acts PROMISC */
	IFLA_NUM_TX_QUEUES,
	IFLA_NUM_RX_QUEUES,
	IFLA_CARRIER,
	IFLA_PHYS_PORT_ID,
	IFLA_CARRIER_CHANGES,
	IFLA_PHYS_SWITCH_ID,
	IFLA_LINK_NETNSID,
	IFLA_PHYS_PORT_NAME,
	IFLA_PROTO_DOWN,
	IFLA_GSO_MAX_SEGS,
	IFLA_GSO_MAX_SIZE,
	IFLA_PAD,
	IFLA_XDP,
	IFLA_EVENT,
	IFLA_NEW_NETNSID,
	IFLA_IF_NETNSID,
	IFLA_CARRIER_UP_COUNT,
	IFLA_CARRIER_DOWN_COUNT,
	IFLA_NEW_IFINDEX,
	__IFLA_MAX,
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum NdaAttrType {
	NDA_UNSPEC,
	NDA_DST,
	NDA_LLADDR,
	NDA_CACHEINFO,
	NDA_PROBES,
	NDA_VLAN,
	NDA_PORT,
	NDA_VNI,
	NDA_IFINDEX,
	NDA_MASTER,
	NDA_LINK_NETNSID,
	NDA_SRC_VNI,
	__NDA_MAX,
}

#[derive(Debug, FromPrimitive)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum NeighborCacheEntryStates {
	NUD_INCOMPLETE = 0x01,
	NUD_REACHABLE = 0x02,
	NUD_STALE = 0x04,
	NUD_DELAY = 0x08,
	NUD_PROBE = 0x10,
	NUD_FAILED = 0x20,
	NUD_NOARP = 0x40,
	NUD_PERMANENT = 0x80,
	NUD_NONE = 0x00,
}

impl std::ops::BitAnd for NeighborCacheEntryStates {
	type Output = u16;
	fn bitand(self, rhs: NeighborCacheEntryStates) -> u16 {
		self as u16 & rhs as u16
	}
}

impl std::ops::BitAnd<NeighborCacheEntryStates> for u16 {
	type Output = u16;
	fn bitand(self, rhs: NeighborCacheEntryStates) -> u16 {
		self & rhs as u16
	}
}

pub fn nlmsg_ok(nlp: *const nlmsghdr, len: c_int) -> bool {
	unsafe { c::nlmsg_ok(nlp, len) }
}

pub fn nlmsg_next(nlp: *const nlmsghdr, len: &mut c_int) -> *const nlmsghdr {
	unsafe { c::nlmsg_next(nlp, len) }
}

pub fn nlmsg_data_rtmsg(nlp: *const nlmsghdr) -> *const rtmsg {
	unsafe { mem::transmute(c::nlmsg_data(nlp)) }
}

pub fn nlmsg_data_ifaddrmsg(nlp: *const nlmsghdr) -> *const ifaddrmsg {
	unsafe { mem::transmute(c::nlmsg_data(nlp)) }
}

pub fn nlmsg_data_ifinfomsg(nlp: *const nlmsghdr) -> *const ifinfomsg {
	unsafe { mem::transmute(c::nlmsg_data(nlp)) }
}

pub fn nlmsg_data_ndmsg(nlp: *const nlmsghdr) -> *const ndmsg {
	unsafe { mem::transmute(c::nlmsg_data(nlp)) }
}

pub fn rtm_rta(rtp: *const rtmsg) -> *const rtattr {
	unsafe { c::rtm_rta(rtp) }
}

pub fn ifa_rta(ifa: *const ifaddrmsg) -> *const rtattr {
	unsafe { c::ifa_rta(ifa) }
}

pub fn ifla_rta(ifi: *const ifinfomsg) -> *const rtattr {
	unsafe { c::ifla_rta(ifi) }
}

pub fn nda_rta(nda: *const ndmsg) -> *const rtattr {
	unsafe { c::nda_rta(nda) }
}

pub fn rtm_payload(nlp: *const nlmsghdr) -> i32 {
	unsafe { c::rtm_payload(nlp) }
}

pub fn ifa_payload(nlp: *const nlmsghdr) -> i32 {
	unsafe { c::ifa_payload(nlp) }
}

pub fn ifla_payload(nlp: *const nlmsghdr) -> i32 {
	unsafe { c::ifla_payload(nlp) }
}

pub fn nda_payload(nlp: *const nlmsghdr) -> i32 {
	unsafe { c::nda_payload(nlp) }
}

pub fn rta_ok(atp: *const rtattr, len: c_int) -> bool {
	unsafe { c::rta_ok(atp, len) }
}

pub fn rta_next(atp: *const rtattr, len: &mut c_int) -> *const rtattr {
	unsafe { c::rta_next(atp, len) }
}

pub fn rta_data(atp: *const rtattr) -> *const c_void {
	unsafe { c::rta_data(atp) }
}

pub fn indextoname(ifindex: i32) -> Option<String> {
	let mut buf: Vec<c_char> = vec![0; 16];

	let mut ptr = &mut buf[0] as *mut c_char;

	unsafe {
		ptr = c::if_indextoname(ifindex as c_int, ptr);
		if ptr.is_null() {
			None
		} else {
			match CStr::from_ptr(ptr as *mut i8).to_str() {
				Ok(s) => Some(String::from(s)),
				Err(_) => None,
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_if_indextoname_get_lo() {
		if let Some(name) = indextoname(1) {
			assert_eq!(name, "lo");
		} else {
			assert!(false);
		}
	}

	#[test]
	fn test_if_indextoname_get_error() {
		if let None = indextoname(212) {
			assert!(true);
		} else {
			assert!(false);
		}
	}
}
