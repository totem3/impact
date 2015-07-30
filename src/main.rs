#![feature(convert)]
extern crate regex;
mod base;
mod resource;
mod query;
use resource::{Record, ResourceRecord, RecordType, RecordClass};
use query::{Query, QueryFlag, QR, Operation, ResponseCode, QuestionRecord};


use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::env;
use std::process::exit;
use std::fmt;
use std::fmt::{Formatter};
use std::convert::From;

fn parse_resolv_conf() -> Vec<String> {
    let mut file = match File::open("/etc/resolv.conf") {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let re = Regex::new(r"(?m:^nameserver (?P<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}))").unwrap();

    let mut nameservers = Vec::new();
    for cap in re.captures_iter(content.as_str()) {
        nameservers.push(String::from(cap.at(1).unwrap_or("")));
    }

    return nameservers;
}

fn usage() {
    println!("usage: impact [name]");
}

fn as_u8(n: u16) -> [u8; 2] {
    let msb = (n >> 8) as u8;
    let lsb = ((n & 0b11111111) as u8);
    [msb, lsb]
}

fn main() {
    let query = Query {
      identity: 0,
      flag: QueryFlag{
          query_or_response: QR::Query,
          operation: Operation::StandardQuery,
          authorative: false,
          truncation: false,
          recursion_disired: true,
          recursion_available: false,
          response_code: ResponseCode::NoError,
      },
      question_count: 1,
      answer_pr_count: 0,
      authorative_pr_count: 0,
      additional_pr_count: 0,
      question_record: vec![QuestionRecord{
          domain_name: "google.com",
          query_type: RecordType::A,
          query_class: RecordClass::IN,
      }],
    };
}

// fn main2() {
//
//     let v: Record<Ipv4Addr> = Record::<Ipv4Addr>::newARecord(
//         String::from("10.10.0.101"),
//         resource::RecordClass::IN,
//         Ipv4Addr::new(10,10,0,101)
//     );
//
//     let v2 = Record::<String>::newCNAMERecord(
//         String::from("10.10.0.101"),
//         resource::RecordClass::IN,
//         String::from("hogehoge.jp")
//     );
//
//     println!("v  = {}", v);
//     println!("v2 = {}", v2);
//     exit(0);
//
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         usage();
//         exit(0);
//     }
//
//     let name = args[1].clone();
//     let names = name.split(".");
//     let mut query_name = Vec::new();
//     for n in names {
//         let length = n.len();
//         query_name.push(length as u8);
//         query_name.extend(n.as_bytes());
//     }
//     query_name.push(0);
//
//     let nameservers = parse_resolv_conf();
//     if nameservers.is_empty() {
//         panic!("no nameservers are found")
//     }
//
//     let local = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
//
//     for nameserver in nameservers {
//         let socket = match UdpSocket::bind(local) {
//             Ok(socket) => socket,
//             Err(e) => {
//                 println!("Failed to bind socket: {}", e);
//                 continue;
//             }
//         };
//
//         let mut buf = [0; 1024];
//
//         let ident         : [u8; 2] = [0, 0];
//         let flag          : [u8; 2] = [0, 0];
//         let q_num         : [u8; 2] = [0, 1];
//         let a_num         : [u8; 2] = [0, 0];
//         let authorize_pr  : [u8; 2] = [0, 0];
//         let additional_pr : [u8; 2] = [0, 0];
//         let q_type        : [u8; 2] = [0, 1];
//         let q_class       : [u8; 2] = [0, 1];
//
//         let mut vec = Vec::new();
//         vec.extend(&ident);
//         vec.extend(&flag);
//         vec.extend(&q_num);
//         vec.extend(&a_num);
//         vec.extend(&authorize_pr);
//         vec.extend(&additional_pr);
//         vec.extend(&query_name);
//         vec.extend(&q_type);
//         vec.extend(&q_class);
//
//         let mut remote = nameserver;
//         remote.push_str(":53");
//         match socket.send_to(&vec.as_slice(), remote.as_str()) {
//             Ok(_) => {},
//             Err(e) => {
//                 println!("err {}", e);
//                 continue;
//             }
//         }
//
//         let res = socket.recv_from(&mut buf);
//         let (amt, _) = res.unwrap();
//
//         let mut response = buf.iter().take(amt);
//         let original_response = response.clone();
//
//         let res_ident : u32 = ((*(response.next().expect("Parse Error")) as u32) << 8) +
//                               (*(response.next().expect("Parse Error")) as u32);
//         let res_flag1 = response.next().expect("Parse Error");
//         let qr = match res_flag1 & (1 << 7) {
//             0 => "Question",
//             _ => "Answer",
//         };
//         let opcode = match (res_flag1 & (0b01111000)) >> 3 {
//             0 => "Standard Query",
//             1 => "Inverse Query",
//             2 => "Server Status Request",
//             n => panic!("Unknown code: {}", n),
//         };
//         let aa = match res_flag1 & 0b00000100 {
//             0 => false,
//             _ => true,
//         };
//         let tc = match res_flag1 & 0b00000010 {
//             0 => false,
//             _ => true,
//         };
//         let rd = match res_flag1 & 0b00000001 {
//             0 => false,
//             _ => true,
//         };
//
//         let res_flag0 = response.next().expect("Parse Error");
//         let ra = match res_flag0 & 0b10000000 {
//             0 => false,
//             _ => true,
//         };
//         let response_code = match res_flag0 & 0b00001111 {
//             0 => "No Error",
//             1 => "Format Error",
//             2 => "Server Error",
//             3 => "Name Error",
//             4 => "Unimplemented",
//             5 => "Request Denied",
//             _ => panic!("Unknown Response Code"),
//         };
//
//         println!("Ident: {}", res_ident);
//         println!("QR: {}", qr);
//         println!("OPCODE: {}", opcode);
//         println!("Authorative Answer: {}", aa);
//         println!("Trancation: {}", tc);
//         println!("Recursion Desired: {}", rd);
//         println!("Recursion Available: {}", ra);
//         println!("Response Code: {}", response_code);
//
//         let res_question_num : u32 = ((*(response.next().expect("Parse Error")) as u32) << 8) +
//                                      (*(response.next().expect("Parse Error")) as u32);
//         let res_answer_pr_num : u32 = ((*(response.next().expect("Parse Error")) as u32) << 8) +
//                                       (*(response.next().expect("Parse Error")) as u32);
//         let res_authorative_pr_num : u32 = ((*(response.next().expect("Parse Error")) as u32) << 8) +
//                                            (*(response.next().expect("Parse Error")) as u32);
//         let res_additional_pr_num : u32 = ((*(response.next().expect("Parse Error")) as u32) << 8) +
//                                           (*(response.next().expect("Parse Error")) as u32);
//         println!("Question Num {}", res_question_num);
//         println!("Answer PR Num {}", res_answer_pr_num);
//         println!("Authorative PR Num {}", res_authorative_pr_num);
//         println!("Additional PR Num {}", res_additional_pr_num);
//
//         // Question Section
//
//         let mut res_names = Vec::new();
//         loop {
//             let label = response.next();
//             match label {
//                 Some(v) if *v > 0 => {},
//                 _ => break,
//             }
//             let len = label.unwrap();
//
//             let mut vec = Vec::new();
//             let res = response.clone();
//             for s in res.take(*len as usize) {
//                 vec.push(*s);
//             }
//             let value : String = String::from_utf8(vec).unwrap();
//             res_names.push(value);
//             res_names.push(String::from("."));
//
//             // skip
//             for _ in 0..*len {
//                 response.next();
//             }
//         }
//
//         println!("");
//         println!("QUESTION");
//         for n in res_names {
//             print!("{}", n);
//         }
//
//         let res_question_type = match ((*(response.next().expect("Parse Error")) as u32) << 8) + (*(response.next().expect("Parse Error")) as u32) {
//             1 => "A",
//             2 => "NS",
//             5 => "CNAME",
//             12 => "PTR",
//             15 => "MX",
//             28 => "AAAA",
//             255 => "ANY",
//             n => panic!("Unknown Question Type: {}", n),
//         };
//         let res_question_class = match ((*(response.next().expect("Parse Error")) as u32) << 8) + (*(response.next().expect("Parse Error")) as u32) {
//             1 => "IN",
//             n => panic!("Unknown Question Class: {}", n),
//         };
//         print!("	{}	{}", res_question_class, res_question_type);
//         println!("");
//
//         // answer
//         println!("");
//         println!("ANSWER");
//         for _ in 0 .. res_answer_pr_num {
//             let field = response.next().expect("Parse Error");
//             let mut name: Vec<u8> = Vec::new();
//             if field & 0b11000000 == 0b11000000 {
//                 let pointer: u32 = (((field & 0b00111111) as u32) << 8) + *(response.next().expect("Parse Error")) as u32;
//                 let orig = original_response.clone();
//                 let mut cnt = 0;
//                 for s in orig.skip(pointer as usize) {
//                     if cnt == 0 {
//                         if *s == 0 {
//                             break;
//                         } else {
//                             cnt = *s;
//                         }
//                     } else {
//                         name.push(*s);
//                         cnt = cnt - 1;
//                         if cnt == 0 {
//                             name.push(46);
//                         }
//                     }
//                 }
//             } else {
//             }
//
//             // println!("name {}", String::from_utf8(name).unwrap());
//
//             let record_type = RecordType::from(((*(response.next().expect("Parse Error")) as u16) << 8) + (*(response.next().expect("Parse Error")) as u16));
//             // println!("RecordType {}", record_type);
//
//             let record_class = ((*(response.next().expect("Parse Error")) as u16) << 8) + (*(response.next().expect("Parse Error")) as u16);
//             if record_class != 1 {
//                 panic!("Unknown Record Class");
//             }
//
//
//             let ttl = ((*(response.next().expect("Parse Error")) as u32) << 24) +
//                       ((*(response.next().expect("Parse Error")) as u32) << 16) +
//                       ((*(response.next().expect("Parse Error")) as u32) <<  8) +
//                       ((*(response.next().expect("Parse Error")) as u32) <<  0);
//             // println!("TTL {}", ttl);
//
//             let rdata_length = ((*(response.next().expect("Parse Error")) as u16) << 8) + (*(response.next().expect("Parse Error")) as u16);
//             // println!("Length {}", rdata_length);
//
//             let rdata = match record_type {
//                 RecordType::A => {
//                     ((*(response.next().expect("Parse Error")) as u32) << 24) +
//                     ((*(response.next().expect("Parse Error")) as u32) << 16) +
//                     ((*(response.next().expect("Parse Error")) as u32) <<  8) +
//                     ((*(response.next().expect("Parse Error")) as u32) <<  0)
//                 },
//                 _ => {
//                     5
//                 }
//             };
//             let addr = Ipv4Addr::from(rdata);
//             let name = String::from_utf8(name).unwrap();
//             println!("{}	{}	IN	{}	{}", name, ttl, record_type, addr);
//         }
//         break;
//     }
//
//
// }
