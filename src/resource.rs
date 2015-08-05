use std::net::{Ipv4Addr, Ipv6Addr};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug,PartialEq)]
pub struct Resource {
    pub name: String,
    pub rtype: ResourceType,
    pub rclass: ResourceClass,
    pub ttl: u32,
    pub rdata: RData,
}

#[derive(Debug,PartialEq)]
pub enum RData {
    A(Ipv4Addr),
    NS(String),
    CNAME(String),
    AAAA(Ipv6Addr)
}

impl Display for RData {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            RData::A(ip) => fmt.write_fmt(format_args!("{}", ip)),
            RData::NS(ref ns) => fmt.write_fmt(format_args!("{}", ns)),
            RData::CNAME(ref cname) => fmt.write_fmt(format_args!("{}", cname)),
            RData::AAAA(ipv6) => fmt.write_fmt(format_args!("{}", ipv6)),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum ResourceType {
    A     = 1,
    NS    = 2,
    CNAME = 5,
    SOA   = 6,
    WKS   = 11,
    PTR   = 12,
    MX    = 15,
    SRV   = 33,
    AAAA  = 38,
}


#[derive(Clone,Debug,PartialEq)]
pub enum ResourceClass {
    IN = 1,
}
