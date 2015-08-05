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

impl ResourceType {
    pub fn from_string(v: String) -> Option<ResourceType> {
        match v.as_ref() {
            "A"     => Some(ResourceType::A),
            "NS"    => Some(ResourceType::NS),
            "CNAME" => Some(ResourceType::CNAME),
            "SOA"   => Some(ResourceType::SOA),
            "WKS"   => Some(ResourceType::WKS),
            "PTR"   => Some(ResourceType::PTR),
            "MX"    => Some(ResourceType::MX),
            "SRV"   => Some(ResourceType::SRV),
            "AAAA"  => Some(ResourceType::AAAA),
            _       => None,
        }
    }
}


#[derive(Clone,Debug,PartialEq)]
pub enum ResourceClass {
    IN = 1,
}
