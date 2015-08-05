#![feature(convert)]
extern crate regex;
mod message;
mod resolver;
mod resource;
mod binary;

use resolver::Resolver;
use resource::ResourceType;
use std::env;
use std::process::exit;

fn usage() {
    println!("usage: impact [name]");
}

fn main() {
    let mut args = env::args();
    args.next();
    if args.len() < 1 {
        usage();
        exit(0);
    }

    let name = args.next().unwrap();
    let rtype = match args.next() {
        Some(v) => match ResourceType::from_string(v) {
            Some(t) => t,
            None => ResourceType::A,
        },
        None => ResourceType::A,
    };

    let resolver = Resolver::from_reolv_conf();
    let response = resolver.resolve(name, rtype);
    match response {
        Ok(message) => {
            println!("Question: ");
            for q in message.question_record {
                println!("{}	{:?}	{:?}", q.domain_name, q.query_class, q.query_type);
            };
            println!("");
            println!("Answer: ");
            for ans in message.answer_record {
                println!("{}	{}	{:?}	{:?}	{}", ans.name, ans.ttl, ans.rclass, ans.rtype, ans.rdata);
            };
        },
        Err(e) => {
            println!("failed to resolve: {}", e);
        },
    }
}
