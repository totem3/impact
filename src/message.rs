use resource::{Resource, ResourceType, ResourceClass, RData};
use binary::encoder::{Encoder, EncodeResult, Encodable};
use std::char;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug,PartialEq)]
pub struct Message {
    pub identity             : u16,
    pub flag                 : Flag,
    pub question_count       : u16,
    pub answer_pr_count      : u16,
    pub authorative_pr_count : u16,
    pub additional_pr_count  : u16,
    pub question_record      : Vec<QuestionRecord>,
    pub answer_record        : Vec<Resource>,
    pub authorative_record   : Vec<Resource>,
    pub additional_record    : Vec<Resource>,
}

impl Message {
    pub fn new(id: u16,
               operation: Operation,
               recursive: bool,
               names: Vec<String>,
               query_type: ResourceType) -> Message {
        let flag = Flag{
            query_or_response: QR::Query,
            operation: operation,
            authorative: false,
            truncation: false,
            recursion_desired: recursive,
            recursion_available: false,
            response_code: ResponseCode::NoError,
        };
        let mut records = Vec::new();
        let names_count = names.len() as u16;
        for name in names {
            let record = QuestionRecord{
                domain_name: name,
                query_type: query_type.clone(),
                query_class: ResourceClass::IN,
            };
            records.push(record);
        }
        Message{
            identity: id,
            flag: flag,
            question_count: names_count,
            answer_pr_count: 0,
            authorative_pr_count: 0,
            additional_pr_count: 0,
            question_record: records,
            answer_record: Vec::new(),
            authorative_record: Vec::new(),
            additional_record: Vec::new(),
        }
    }

    fn read_u8(idx: &mut usize, data: &[u8]) -> u8 {
        let u = data[*idx];
        *idx = *idx + 1;
        u
    }
    fn read_u16(idx: &mut usize, data: &[u8]) -> u16 {
        let a = (data[*idx] as u16) << 8;
        *idx = *idx + 1;
        let b = data[*idx] as u16;
        *idx = *idx + 1;
        a | b
    }
    fn read_u32(idx: &mut usize, data: &[u8]) -> u32 {
        let a = (data[*idx] as u32) << 24;
        *idx = *idx + 1;
        let b = (data[*idx] as u32) << 16;
        *idx = *idx + 1;
        let c = (data[*idx] as u32) << 8;
        *idx = *idx + 1;
        let d = data[*idx] as u32;
        *idx = *idx + 1;
        a | b | c | d
    }
    fn read_label(idx: &mut usize, data: &[u8]) -> Result<String, DecodeError> {
        let mut split = Vec::new();
        while data[*idx] != 0 {
            let len = data[*idx];
            *idx = *idx + 1;
            let mut part = String::new();
            for _ in 0..len {
                match char::from_u32(data[*idx] as u32) {
                    Some(c) => part.push(c),
                    None    => return Err(DecodeError::InvalidFormatErr("Invalid Name")),
                }
                *idx = *idx + 1;
            }
            split.push(part);
        }
        *idx = *idx + 1;
        let name = split.join(&".");
        Ok(name)
    }
    fn read_question_record(idx: &mut usize, data: &[u8]) -> Result<QuestionRecord, DecodeError> {
        let mut idx = idx;
        let name = match Message::read_label(idx, data) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let record_type = match Message::read_u16(&mut idx, data) {
            1  => ResourceType::A,
            2  => ResourceType::NS,
            5  => ResourceType::CNAME,
            6  => ResourceType::SOA,
            11 => ResourceType::WKS,
            12 => ResourceType::PTR,
            15 => ResourceType::MX,
            33 => ResourceType::SRV,
            38 => ResourceType::AAAA,
            _  =>  return Err(DecodeError::InvalidFormatErr("Unknown or Not Supported Resource Type")),
        };
        let record_class = match Message::read_u16(&mut idx, data) {
            1 => ResourceClass::IN,
            _  => return Err(DecodeError::InvalidFormatErr("Unknown or Not Supported Resource Class"))
        };
        let record = QuestionRecord{
            domain_name: name,
            query_type: record_type,
            query_class: record_class,
        };
        Ok(record)
    }
    fn read_resource_record(idx: &mut usize, data: &[u8]) -> Result<Resource, DecodeError> {
        let mut idx = idx;
        let name = if data[*idx] & 0xc0 == 0xc0 {
            let msb = ((data[*idx] & 0x3f) as u16) << 8;
            *idx = *idx + 1;
            let lsb = data[*idx] as u16;
            let mut pointer: usize = (msb | lsb) as usize;
            let res = match Message::read_label(&mut pointer, data) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            *idx = *idx + 1;
            res
        } else {
            match Message::read_label(&mut idx, data) {
                Ok(v) => v,
                Err(e) => return Err(e),
            }
        };
        let record_type = match Message::read_u16(&mut idx, data) {
            1  => ResourceType::A,
            2  => ResourceType::NS,
            5  => ResourceType::CNAME,
            6  => ResourceType::SOA,
            11 => ResourceType::WKS,
            12 => ResourceType::PTR,
            15 => ResourceType::MX,
            33 => ResourceType::SRV,
            38 => ResourceType::AAAA,
            _  => return Err(DecodeError::InvalidFormatErr("Unknown or Not Supported Resource Type")),
        };
        let record_class = match Message::read_u16(&mut idx, data) {
            1 => ResourceClass::IN,
            _  => return Err(DecodeError::InvalidFormatErr("Unknown or Not Supported Resource Class"))
        };
        let ttl = Message::read_u32(&mut idx, data);
        let _ = Message::read_u16(&mut idx, data); // TODO use length
        let rdata = match record_type {
            ResourceType::A => {
                let mut idx = idx;
                RData::A(Ipv4Addr::from(Message::read_u32(&mut idx, data)))
            },
            ResourceType::CNAME => {
                let mut idx = idx;
                let name = if data[*idx] & 0xc0 == 0xc0 {
                        let mut pointer: usize = (data[*idx] & 0x3f) as usize;
                        let res = match Message::read_label(&mut pointer, data) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                        *idx = *idx + 1;
                        res
                    } else {
                        match Message::read_label(&mut idx, data) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        }
                    };
                RData::CNAME(name)
            },
            ResourceType::AAAA => {
                let mut idx = idx;
                RData::AAAA(Ipv6Addr::new(
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                    Message::read_u16(&mut idx, data),
                ))
            },
            _ => panic!("not supprted"),
        };
        let resource = Resource {
            name: name,
            rtype: record_type,
            rclass: record_class,
            ttl: ttl,
            rdata: rdata,
        };
        Ok(resource)
    }
}

pub enum DecodeError {
    InvalidFormatErr(&'static str),
}

impl<'a> Message {
    pub fn decode(data: &'a [u8]) -> Result<Message, DecodeError> {
        let mut idx = 0;
        let id = Message::read_u16(&mut idx, data);
        let flag_msb = Message::read_u8(&mut idx, data);
        let qr = if flag_msb & 0x80 == 0x80 {
            QR::Response
        } else {
            QR::Query
        };
        let op = match flag_msb & 0x78 {
            0 => Operation::StandardQuery,
            1 => Operation::InverseQuery,
            2 => Operation::ServerStatusRequest,
            n => {
                println!("Operation: {:?}", n);
                return Err(DecodeError::InvalidFormatErr("Unknown Operation"));
            },
        };
        let aa = flag_msb & 0x04 == 0x04;
        let tc = flag_msb & 0x02 == 0x02;
        let rd = flag_msb & 0x01 == 0x01;
        let flag_lsb = Message::read_u8(&mut idx, data);
        let ra = flag_lsb & 0x80 == 0x80;
        let rcode = match flag_lsb & 0x0f  {
             0 => ResponseCode::NoError,
             1 => ResponseCode::FormatError,
             2 => ResponseCode::ServerError,
             3 => ResponseCode::NameError,
             4 => ResponseCode::NotImplementedError,
             5 => ResponseCode::RequestDenied,
             _ => return Err(DecodeError::InvalidFormatErr("Unknown Response Code")),
        };
        let flag = Flag {
            query_or_response: qr,
            operation: op,
            authorative: aa,
            truncation: tc,
            recursion_desired: rd,
            recursion_available: ra,
            response_code: rcode,
        };

        let question_count = Message::read_u16(&mut idx, data);
        let answer_count = Message::read_u16(&mut idx, data);
        let authorative_count = Message::read_u16(&mut idx, data);
        let additional_count = Message::read_u16(&mut idx, data);
        let mut question_records = Vec::new();
        for _ in 0..question_count {
            let question_record = Message::read_question_record(&mut idx, data);
            match question_record {
                Ok(v)  => question_records.push(v),
                Err(e) => return Err(e),
            };
        }
        let mut answer_records = Vec::new();
        for _ in 0..answer_count {
            let answer_record = Message::read_resource_record(&mut idx, data);
            match answer_record {
                Ok(v)  => answer_records.push(v),
                Err(e) => return Err(e),
            };
        }
        let mut authorative_records = Vec::new();
        for _ in 0..authorative_count {
            let authorative_record = Message::read_resource_record(&mut idx, data);
            match authorative_record {
                Ok(v)  => authorative_records.push(v),
                Err(e) => return Err(e),
            };
        }
        let mut additional_records = Vec::new();
        for _ in 0..additional_count {
            let additional_record = Message::read_resource_record(&mut idx, data);
            match additional_record {
                Ok(v)  => additional_records.push(v),
                Err(e) => return Err(e),
            };
        }
        let message = Message{
            identity: id,
            flag: flag,
            question_count: question_count,
            answer_pr_count: answer_count,
            authorative_pr_count: authorative_count,
            additional_pr_count: additional_count,
            question_record: question_records,
            answer_record: answer_records,
            authorative_record: authorative_records,
            additional_record: additional_records,
        };
        Ok(message)
    }
}

impl Encodable for Message {
    fn encode(&self, encoder: &mut Encoder) -> EncodeResult<()> {
        try!(encoder.emit_u16(self.identity));
        try!(self.flag.encode(encoder));
        try!(encoder.emit_u16(self.question_count));
        try!(encoder.emit_u16(self.answer_pr_count));
        try!(encoder.emit_u16(self.authorative_pr_count));
        try!(encoder.emit_u16(self.additional_pr_count));
        encoder.emit_vec(&self.question_record)
    }
}

#[derive(Debug, PartialEq)]
pub struct Flag {
    pub query_or_response: QR,
    pub operation: Operation,
    pub authorative: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub response_code: ResponseCode,
}

impl Encodable for Flag {
    fn encode(&self, encoder: &mut Encoder) -> EncodeResult<()> {
        let mut msb = 0u8;
        let qr = (self.query_or_response.clone() as u8) << 7;
        msb = msb | qr;

        let op = (self.operation.clone() as u8) << 3;
        msb = msb | op;

        if self.recursion_desired {
            msb = msb | 0b00000001;
        }
        let lsb = 0u8;
        match encoder.emit_u8(msb) {
            Err(s) => return Err(s),
            _ => {},
        };
        encoder.emit_u8(lsb)
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum QR {
    Query    = 0,
    Response = 1,
}

#[derive(Clone,Debug,PartialEq)]
pub enum Operation {
    StandardQuery       = 0,
    InverseQuery        = 1,
    ServerStatusRequest = 2,
}
#[derive(Clone,Debug,PartialEq)]
pub enum ResponseCode {
    NoError             = 0,
    FormatError         = 1,
    ServerError         = 2,
    NameError           = 3,
    NotImplementedError = 4,
    RequestDenied       = 5,
}

#[derive(Debug,PartialEq)]
pub struct QuestionRecord {
    pub domain_name: String,
    pub query_type: ResourceType,
    pub query_class: ResourceClass,
}

impl Encodable for QuestionRecord {
    fn encode(&self, encoder: &mut Encoder) -> EncodeResult<()> {
        let name = self.domain_name.clone();
        let sp = name.split(".");
        for s in sp {
            let length = s.len();
            try!(encoder.emit_u8(length as u8));
            try!(encoder.emit_str(s));
        }
        try!(encoder.emit_u8(0));

        try!(encoder.emit_u16(self.query_type.clone() as u16));
        encoder.emit_u16(self.query_class.clone() as u16)
    }
}

#[cfg(test)]
mod test {
    use super::{Message, Flag, QR, Operation, ResponseCode, QuestionRecord, DecodeError};
    use binary::encoder;
    use binary::encoder::{Encoder, Encodable};
    use resource::{Resource, ResourceType, ResourceClass, RData};
    use std::net::Ipv4Addr;

    #[test]
    fn test_query_encode() {
        let query = Message::new(
            0,
            Operation::StandardQuery,
            true,
            vec![String::from("google.com")],
            ResourceType::A
        );
        let encoded = encoder::encode(&query);
        let expected = vec![
            0u8, 0u8, // ident 0
            1u8, 0u8, // flag recursion_desired true
            0u8, 1u8, // question num 1
            0u8, 0u8, // answer num 0
            0u8, 0u8, // authorative num 0
            0u8, 0u8, // additional num 0
            // question google.com IN A
            6u8, 103u8, 111u8, 111u8, 103u8, 108u8, 101u8,
            3u8, 99u8, 111u8, 109u8,
            0u8, // name end
            0u8, 1u8, // type
            0u8, 1u8, // class
        ];
        assert_eq!(encoded.unwrap(), expected);
    }

    #[test]
    fn test_decode_question_record() {
        let mut encoded = [
            0u8, 0u8, // ident 0
            1u8, 0u8, // flag recursion_desired true
            0u8, 1u8, // question num 1
            0u8, 0u8, // answer num 0
            0u8, 0u8, // authorative num 0
            0u8, 0u8, // additional num 0
            // question google.com IN A
            6u8, 103u8, 111u8, 111u8, 103u8, 108u8, 101u8,
            3u8, 99u8, 111u8, 109u8,
            0u8, // name end
            0u8, 1u8, // type
            0u8, 1u8, // class
        ];
        let decoded = Message::decode(&mut encoded);
        let question_record = QuestionRecord {
            domain_name: String::from("google.com"),
            query_type: ResourceType::A,
            query_class: ResourceClass::IN,
        };
        let expected = Message{
            identity: 0,
            flag: Flag{
                query_or_response: QR::Query,
                operation: Operation::StandardQuery,
                authorative: false,
                truncation: false,
                recursion_desired: true,
                recursion_available: false,
                response_code: ResponseCode::NoError,
            },
            question_count: 1,
            answer_pr_count: 0,
            authorative_pr_count: 0,
            additional_pr_count: 0,
            question_record: vec![question_record],
            answer_record: Vec::new(),
            authorative_record: Vec::new(),
            additional_record: Vec::new(),
        };
        match decoded {
            Ok(v) => assert_eq!(v, expected),
            Err(DecodeError::InvalidFormatErr(s)) => {
                println!("Error {}", s);
                assert!(false)
            },
        }
    }

    #[test]
    fn test_decode_resource_record() {
        let mut encoded = [
            0u8, 0u8, // ident 0
            1u8, 0u8, // flag recursion_desired true
            0u8, 1u8, // question num 1
            0u8, 1u8, // answer num 1
            0u8, 0u8, // authorative num 0
            0u8, 0u8, // additional num 0
            // question google.com IN A
            6u8, 103u8, 111u8, 111u8, 103u8, 108u8, 101u8,
            3u8, 99u8, 111u8, 109u8,
            0u8, // name end
            0u8, 1u8, // type
            0u8, 1u8, // class
            0xc0, 0x0c,
            0x00, 0x01,
            0x00, 0x01,
            0x00, 0x00, 0x00, 0x63, // ttl
            0x00, 0x04, // rdata
            0xad, 0xc2, 0x7e, 0xc1, // ip
        ];
        let decoded = Message::decode(&mut encoded);
        let question_record = QuestionRecord {
            domain_name: "google.com".to_string(),
            query_type: ResourceType::A,
            query_class: ResourceClass::IN,
        };
        let resource_record = Resource {
            name: "google.com".to_string(),
            rtype: ResourceType::A,
            rclass: ResourceClass::IN,
            ttl: 99,
            rdata: RData::A(Ipv4Addr::new(173, 194, 126, 193)),
        };
        let expected = Message{
            identity: 0,
            flag: Flag{
                query_or_response: QR::Query,
                operation: Operation::StandardQuery,
                authorative: false,
                truncation: false,
                recursion_desired: true,
                recursion_available: false,
                response_code: ResponseCode::NoError,
            },
            question_count: 1,
            answer_pr_count: 1,
            authorative_pr_count: 0,
            additional_pr_count: 0,
            question_record: vec![question_record],
            answer_record: vec![resource_record],
            authorative_record: Vec::new(),
            additional_record: Vec::new(),
        };
        match decoded {
            Ok(v) => assert_eq!(v, expected),
            Err(DecodeError::InvalidFormatErr(s)) => {
                println!("Error {}", s);
                assert!(false)
            },
        }
    }
}
