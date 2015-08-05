use std::net::{SocketAddrV4, UdpSocket, Ipv4Addr};
use message::{Message, DecodeError, Operation};
use resource::ResourceType;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use regex::Regex;
use binary::encoder;

pub struct Resolver {
    name_servers: Vec<Ipv4Addr>,
}

impl Resolver {
    pub fn new(name_servers: Vec<Ipv4Addr>) -> Resolver {
        Resolver{
            name_servers: name_servers,
        }
    }
    pub fn from_reolv_conf() -> Resolver {
        let ns = Resolver::parse_resolv_conf();
        Resolver::new(ns)
    }
    fn parse_resolv_conf() -> Vec<Ipv4Addr> {
        let mut file = match File::open("/etc/resolv.conf") {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let re = Regex::new(r"(?m:^nameserver (?P<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}))").unwrap();

        let mut nameservers = Vec::new();
        for cap in re.captures_iter(content.as_str()) {
            let s = cap.at(1).unwrap_or("").to_string().to_owned();
            match Ipv4Addr::from_str(&s) {
                Ok(ip) => nameservers.push(ip),
                _ => {},
            }
        }

        return nameservers;
    }
    pub fn resolve(&self,
               name: String,
               resource_type: ResourceType) -> Result<Message, String> {
        let local = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
        for ns in self.name_servers.iter() {
            let name = name.clone();
            let query = Message::new(
                0,
                Operation::StandardQuery,
                true,
                vec![name],
                resource_type,
            );
            let socket = match UdpSocket::bind(local) {
                Ok(sock) => sock,
                Err(e) => {
                    println!("Failed to bind socket: {}", e);
                    continue;
                }
            };

            let mut remote = ns.to_string();
            remote.push_str(":53");
            match socket.send_to(encoder::encode(&query).ok().unwrap().as_slice(), remote.as_str()) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error {}", e);
                    continue;
                }
            };

            let mut buf = [0; 1024];
            let res = socket.recv_from(&mut buf);
            let (len, _) = res.unwrap();

            let response: &[u8] = &buf[0..len];
            return match Message::decode(response) {
                Ok(v) => Ok(v),
                Err(DecodeError::InvalidFormatErr(s)) => Err(s.to_string()),
            }
        }
        Err("Failed to resolve".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Resolver;
    use std::net::Ipv4Addr;
    use std::str::FromStr;
    use std::ops::Index;
    use resource::{ResourceType, ResourceClass, Resource, RData};

    #[test]
    fn test_resolve_localhost() {
        let ns = Resolver::parse_resolv_conf();
        let resolver = Resolver::new(ns);
        let expected = Resource {
            name: "localhost".to_string(),
            rtype: ResourceType::A,
            rclass: ResourceClass::IN,
            ttl: 600,
            rdata: RData::A(Ipv4Addr::new(127, 0, 0, 1)),
        };
        match resolver.resolve("localhost".to_string(), ResourceType::A) {
            Ok(message) => {
                assert_eq!(*message.answer_record.index(0), expected);
            },
            Err(e) => {
                println!("Error {:?}", e);
                assert!(false);
            }
        };
    }
}
