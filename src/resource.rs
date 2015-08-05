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
pub struct SOAData {
    pub primary_ns: String,
    pub admin_mb: String,
    pub serial: u32,
    pub refresh_interval: u32,
    pub retry_interval: u32,
    pub expiration_limit: u32,
    pub minimal_ttl: u32,
}

impl SOAData {
    pub fn new(ns: String, mb: String,
               serial: u32, refresh_interval: u32,
               retry_interval: u32, expiration_limit: u32,
               minimal_ttl: u32) -> SOAData {
        SOAData {
            primary_ns: ns,
            admin_mb: mb,
            serial: serial,
            refresh_interval: refresh_interval,
            retry_interval: retry_interval,
            expiration_limit: expiration_limit,
            minimal_ttl: minimal_ttl,
        }
    }
}

impl Display for SOAData {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{} {} {} {} {} {} {}",
            self.primary_ns,
            self.admin_mb,
            self.serial,
            self.refresh_interval,
            self.retry_interval,
            self.expiration_limit,
            self.minimal_ttl
        ))
    }
}

#[derive(Debug,PartialEq)]
pub struct MXData {
    pub preference: u16,
    pub mx: String,
}

impl MXData {
    pub fn new(pref: u16, mx: String) -> MXData {
        MXData {
            preference: pref,
            mx: mx,
        }
    }
}

impl Display for MXData {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{} {}", self.preference, self.mx))
    }
}

#[derive(Debug,PartialEq)]
pub enum RData {
    A(Ipv4Addr),
    NS(String),
    CNAME(String),
    AAAA(Ipv6Addr),
    SOA(SOAData),
    MX(MXData),
    PTR(String),
}

impl Display for RData {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            RData::A(ip) => fmt.write_fmt(format_args!("{}", ip)),
            RData::NS(ref ns) => fmt.write_fmt(format_args!("{}", ns)),
            RData::CNAME(ref cname) => fmt.write_fmt(format_args!("{}", cname)),
            RData::AAAA(ipv6) => fmt.write_fmt(format_args!("{}", ipv6)),
            RData::SOA(ref soa) => fmt.write_fmt(format_args!("{}", soa)),
            RData::MX(ref mx) => fmt.write_fmt(format_args!("{}", mx)),
            RData::PTR(ref ptr) => fmt.write_fmt(format_args!("{}", ptr)),
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
    AAAA  = 28,
    SRV   = 33,
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
