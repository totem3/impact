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
        for b in response {
            print!("{:0>2x} ", b);
        }
        println!("");
        break;
    }

}
