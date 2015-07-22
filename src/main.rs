#![feature(convert)]
extern crate regex;
use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::env;
use std::process::exit;

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

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        exit(0);
    }

    let name = args[1].clone();
    let names = name.split(".");
    let mut query_name = Vec::new();
    for n in names {
        let length = n.len();
        query_name.push(length as u8);
        query_name.extend(n.as_bytes());
    }
    query_name.push(0);

    let nameservers = parse_resolv_conf();
    if nameservers.is_empty() {
        panic!("no nameservers are found")
    }

    let local = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);

    for nameserver in nameservers {
        let socket = match UdpSocket::bind(local) {
            Ok(socket) => socket,
            Err(e) => {
                println!("Failed to bind socket: {}", e);
                continue;
            }
        };

        let mut buf = [0; 1024];

        let ident         : [u8; 2] = [0, 0];
        let flag          : [u8; 2] = [0, 0];
        let q_num         : [u8; 2] = [0, 1];
        let a_num         : [u8; 2] = [0, 0];
        let authorize_pr  : [u8; 2] = [0, 0];
        let additional_pr : [u8; 2] = [0, 0];
        let q_type        : [u8; 2] = [0, 1];
        let q_class       : [u8; 2] = [0, 1];

        let mut vec = Vec::new();
        vec.extend(&ident);
        vec.extend(&flag);
        vec.extend(&q_num);
        vec.extend(&a_num);
        vec.extend(&authorize_pr);
        vec.extend(&additional_pr);
        vec.extend(&query_name);
        vec.extend(&q_type);
        vec.extend(&q_class);

        let mut remote = nameserver;
        remote.push_str(":53");
        match socket.send_to(&vec.as_slice(), remote.as_str()) {
            Ok(_) => {},
            Err(e) => {
                println!("err {}", e);
                continue;
            }
        }

        let res = socket.recv_from(&mut buf);
        let (amt, _) = res.unwrap();

        let response = buf.iter().take(amt);
        let mut idx = 0;
        let mut res_ident = Vec::new();
        let mut res_flag = Vec::new();
        let mut res_q_num = Vec::new();
        let mut res_a_num = Vec::new();
        let mut res_authorize_pr = Vec::new();
        let mut res_additional_pr = Vec::new();
        for b in response {
            match idx {
                0  | 1  => { res_ident.push(b) },
                2  | 3  => { res_flag.push(b) },
                4  | 5  => { res_q_num.push(b) },
                6  | 7  => { res_a_num.push(b) },
                8  | 9  => { res_authorize_pr.push(b) },
                10 | 11 => { res_additional_pr.push(b) },
                _ => {}
            }
            idx += 1;
        }
        println!("res_ident {:?}", res_ident);
        println!("res_flag {:?}", res_flag);
        let qr_flag = res_flag[0] >> 7;
        let op_code = (res_flag[0] << 1) >> 3;
        let aa = res_flag[0] & (1 << 2) == (1 << 2);
        let tc = res_flag[0] & (1 << 1) == (1 << 1);
        let rd = res_flag[0] & (1 << 0) == (1 << 0);
        let ra = res_flag[1] & (1 << 7) == (1 << 7);
        let reserved = (res_flag[1] << 1) >> 5;
        let response_code = res_flag[1] & 0b1111;
        println!("  qr_flag {:?}", qr_flag);
        println!("  op_code {:?}", op_code);
        println!("  aa {:?}", aa);
        println!("  tc {:?}", tc);
        println!("  rd {:?}", rd);
        println!("  ra {:?}", ra);
        println!("  reserved {:?}", reserved);
        println!("  response_code {:?}", response_code);
        println!("res_q_num {}", res_q_num[1]);
        println!("res_a_num {}", res_a_num[1]);
        println!("res_authorize_pr {}", res_authorize_pr[1]);
        println!("res_additional_pr {}", res_additional_pr[1]);

        break;
    }

}
