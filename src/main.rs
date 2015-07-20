#![feature(collections)]
#![feature(convert)]
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};

fn main() {
    let socket = UdpSocket::bind("192.168.1.1:10099").unwrap();
    let mut buf = [0; 1024];
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let port = 53;
    let addr = SocketAddrV4::new(ip, port);

    let ident = [0u8, 0u8];
    let flag = [0u8, 0u8];
    let q_num = [0u8, 1u8];
    let a_num = [0u8, 0u8];
    let authorize_pr = [0u8, 0u8];
    let additional_pr = [0u8, 0u8];
    let question = "8google3com0";
    let q_type = [0u8, 1u8];
    let q_class = [0u8, 1u8];

    let mut vec = Vec::new();
    vec.push_all(&ident);
    vec.push_all(&flag);
    vec.push_all(&q_num);
    vec.push_all(&a_num);
    vec.push_all(&authorize_pr);
    vec.push_all(&additional_pr);
    vec.push_all(question.as_bytes());
    vec.push_all(&q_type);
    vec.push_all(&q_class);

    let size = socket.send_to(vec.as_slice(), addr);
    println!("send size: {}", size.unwrap());

    let res = socket.recv_from(&mut buf);
    let (amt, src) = res.unwrap();

}
