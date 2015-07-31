use resource::{RecordType, RecordClass};
use binary::decoder::{Decoder, DecodeResult, Decodable};

pub struct Answer<T>{
    domain_name: String,
    record_type: RecordType,
    record_class: RecordClass,
    ttl: u32,
    data_length: u16,
    value: T,
}

