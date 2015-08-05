use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug,PartialEq)]
pub struct Resource {
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
