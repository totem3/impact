use resource::{Resource, ResourceType, ResourceClass};
use binary::encoder::{Encoder, EncodeResult, Encodable};

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
               names: Vec<&'static str>,
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
}

pub enum DecodeError {
    InvalidFormatErr(&'static str),
}

impl<'a> Message {
    fn decode(data: &'a [u8]) -> Result<Message, DecodeError> {
        let orig = data.clone();
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
            _ => return Err(DecodeError::InvalidFormatErr("Unknown Operation")),
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
        let message = Message{
            identity: id,
            flag: flag,
            question_count: question_count,
            answer_pr_count: answer_count,
            authorative_pr_count: authorative_count,
            additional_pr_count: additional_count,
            question_record: Vec::new(),
            answer_record: Vec::new(),
            authorative_record: Vec::new(),
            additional_record: Vec::new(),
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
        let mut lsb = 0u8;
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
    pub domain_name: &'static str,
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
    use resource::{Resource, ResourceType, ResourceClass};

    #[test]
    fn test_query_encode() {
        let query = Message::new(
            0,
            Operation::StandardQuery,
            true,
            vec!["google.com"],
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
    fn test_decode() {
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
            question_record: Vec::new(),
            answer_record: Vec::new(),
            authorative_record: Vec::new(),
            additional_record: Vec::new(),
        };
        match decoded {
            Ok(v) => assert_eq!(v, expected),
            Err(DecodeError::InvalidFormatErr(s)) => assert!(false),
        }
        
    }

}
