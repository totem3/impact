use resource::{Record, ResourceRecord, RecordType, RecordClass};
use binary::encoder::{Encoder, EncodeResult, Encodable};

pub struct Message {
    pub identity: u16,
    pub flag: Flag,
    pub question_count: u16,
    pub answer_pr_count: u16,
    pub authorative_pr_count: u16,
    pub additional_pr_count: u16,
    pub question_record: Vec<QuestionRecord>,
    pub answer_record: Vec<Record>,
    pub authorative_record: Vec<Record>,
    pub additional_record: Vec<Record>,
}

impl Message {
    pub fn new(id: u16,
               operation: Operation,
               recursive: bool,
               names: Vec<&'static str>,
               query_type: RecordType) -> Message {
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
                query_class: RecordClass::IN,
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
        }
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

#[derive(Clone,Debug)]
pub enum QR {
    Query    = 0,
    Response = 1,
}

#[derive(Clone,Debug)]
pub enum Operation {
    StandardQuery       = 0,
    InverseQuery        = 1,
    ServerStatusRequest = 2,
}
#[derive(Clone,Debug)]
pub enum ResponseCode {
    NoError             = 0,
    FormatError         = 1,
    ServerError         = 2,
    NameError           = 3,
    NotImplementedError = 4,
    RequestDenied       = 5,
}

#[derive(Debug)]
pub struct QuestionRecord {
    pub domain_name: &'static str,
    pub query_type: RecordType,
    pub query_class: RecordClass,
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
    use super::{Message, Flag, QR, Operation, ResponseCode, QuestionRecord};
    use binary::encoder;
    use binary::encoder::{Encoder, Encodable};
    use resource::{Record, ResourceRecord, RecordType, RecordClass};

    #[test]
    fn test_query_encode() {
        let query = Message::new(
            0,
            Operation::StandardQuery,
            true,
            vec!["google.com"],
            RecordType::A
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
}
