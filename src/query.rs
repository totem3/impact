use base::id::Identity;
use resource::{RecordType, RecordClass};

pub struct Query {
    pub identity: u16,
    pub flag: QueryFlag,
    pub question_count: u16,
    pub answer_pr_count: u16,
    pub authorative_pr_count: u16,
    pub additional_pr_count: u16,
    pub question_record: Vec<QuestionRecord>,
}

pub struct QueryFlag {
    pub query_or_response: QR,
    pub operation: Operation,
    pub authorative: bool,
    pub truncation: bool,
    pub recursion_disired: bool,
    pub recursion_available: bool,
    pub response_code: ResponseCode,
}

impl QueryFlag {
    pub fn encode(&self) -> Vec<u8> {
        let mut msb = 0u8;
        let qr = (self.query_or_response.clone() as u8) << 7;
        msb = msb & qr;

        let op = (self.operation.clone() as u8) << 3;
        msb = msb & op;

        if self.recursion_disired {
            msb = msb & 1;
        }
        let mut lsb = 0u8;
        vec![msb, lsb]
    }
}

#[derive(Clone,Debug)]
pub enum QR {
    Query    = 0,
    Response = 1,
}
// impl Clone for QR {
//     fn clone(&self) -> Self {
//         match *self {
//             QR::Query => QR::Query,
//             QR::Response => QR::Response,
//         }
//     }
// }

#[derive(Clone,Debug)]
pub enum Operation {
    StandardQuery       = 0,
    InverseQuery        = 1,
    ServerStatusRequest = 2,
}
pub enum ResponseCode {
    NoError             = 0,
    FormatError         = 1,
    ServerError         = 2,
    NameError           = 3,
    NotImplementedError = 4,
    RequestDenied       = 5,
}

pub struct QuestionRecord {
    pub domain_name: &'static str,
    pub query_type: RecordType,
    pub query_class: RecordClass,
}
