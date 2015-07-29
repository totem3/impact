use std::net::Ipv4Addr;
use std::str;
use std::fmt::{Display, Formatter, Error};

pub trait ResourceRecord {
    type V;
    fn name_server(&self) -> &String;
    fn record_type(&self) -> &RecordType;
    fn record_class(&self) -> &RecordClass;
    fn value(&self) -> &Self::V;
}

#[derive(Debug)]
pub struct Record<T: Display> {
    name_server: String,
    record_type: RecordType,
    record_class: RecordClass,
    value: T,
}
impl<T: Display> Record<T> {
    pub fn newARecord(name_server: String,
                      record_class: RecordClass,
                      value: Ipv4Addr) -> Record<Ipv4Addr> {
        Record{
            name_server: name_server,
            record_type: RecordType::A,
            record_class: record_class,
            value: value,
        }
    }
    pub fn newCNAMERecord(name_server: String,
                          record_class: RecordClass,
                          value: String) -> Record<String> {
        Record{
            name_server: name_server,
            record_type: RecordType::CNAME,
            record_class: record_class,
            value: value,
        }
    }
}

impl<T: Display> Display for Record<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let format = format!("{:?}	{:?}	{}", self.record_class, self.record_type, self.value);
        fmt.write_str(&format)
    }
}

impl<T: Display> ResourceRecord for Record<T> {
    type V = T;
    fn name_server(&self) -> &String {
        &self.name_server
    }
    fn record_type(&self) -> &RecordType {
        &self.record_type
    }
    fn record_class(&self) -> &RecordClass {
        &self.record_class
    }
    fn value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub enum RecordType {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    MX,
    SRV,
    AAAA,
}


#[derive(Debug)]
pub enum RecordClass {
    IN,
}
