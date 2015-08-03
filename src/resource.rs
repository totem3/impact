use std::net::{Ipv4Addr, Ipv6Addr};
use std::str;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug,PartialEq)]
pub struct Resource {
    rtype: ResourceType,
    rclass: ResourceClass,
    ttl: u32,
    rdata: RData,
}

#[derive(Debug,PartialEq)]
pub enum RData {
    A(Ipv4Addr),
    NS(String),
    CNAME(String),
    AAAA(Ipv6Addr)
}

#[derive(Clone,Debug,PartialEq)]
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
