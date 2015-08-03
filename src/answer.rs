// use resource::{RecordType, RecordClass};
// use binary::decoder::{Decoder, DecodeResult, Decodable};
// use std::net::Ipv4Addr;
// use std::result::Result;

// pub struct Answer<T>{
//     domain_name  : String,
//     record_type  : RecordType,
//     record_class : RecordClass,
//     ttl          : u32,
//     data_length  : u16,
//     value        : T,
// }

// impl Decodable for Ipv4Addr {
//     fn decode(d: &mut Decoder) -> Result<Self, String> {
//         match d.read_u32() {
//             Ok(v)  => Ok(Ipv4Addr::from(v)),
//             Err(s) => Err(s),
//         }
//     }
// }
