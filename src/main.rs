#![allow(dead_code,unused_variables,unused_imports)]
#![feature(ip)]
use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr, AddrParseError}, str::FromStr};
use clap::Parser;


#[derive(Parser,Default,Debug)]
struct Arguments {
    ip: String,
    // #[clap(short, long)]
    // destination: Option<String>,
}

fn main() {
    let args:Arguments = Arguments::parse();
    
    if !check_if_valid_ip(&args.ip) {
        println!("{}", "Invalid IP".to_string());
        std::process::exit(0)
    }

    if !check_if_global_ip(&args.ip) {
        println!("{}", "Not global IP".to_string());
        std::process::exit(0)
    }
}

fn check_if_valid_ip(s: &String) -> bool {
    let ip = IpAddr::from_str(&s);
    match ip {
        Ok(ip) => true,
        _ => false
    }
}

fn check_if_global_ip(s: &String) -> bool {
    IpAddr::from_str(&s).unwrap().is_global()
}

////////////// Tests /////////////
#[test]
fn test_check_if_valid_ip() {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    
    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));
    
    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);
}