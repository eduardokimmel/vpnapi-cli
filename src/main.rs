#![allow(dead_code,unused_variables,unused_imports)]
#![feature(ip)]
use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr, AddrParseError}, str::FromStr};
use clap::Parser;
// use  serde::{Deserialize, "derive"};


#[derive(Parser,Default,Debug)]
struct Arguments {
    ip: String,
    #[clap(default_value = "",short, long)]
    key: String,
    #[clap(short, long)]
    file: Option<String>,
    #[clap(short, long)]
    config: Option<String>
}

#[tokio::main]
async fn main() {
    let args:Arguments = Arguments::parse();
    
    if !check_if_valid_ip(&args.ip) {
        println!("{}", "Invalid IP".to_string());
        std::process::exit(0)
    }

    if !check_if_global_ip(&args.ip) {
        println!("{}", "Not global IP".to_string());
        std::process::exit(0)
    }

    let key = get_api_key(&args.key);
    if key == "" {
        println!("{}", "No key entered".to_string());
        std::process::exit(0)
    }

    let r = get_vpnapi_result(&args.ip, &key).await;
    println!("result = {:#?}", r);


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

// #[derive(Deserialize, Debug)]
// struct VpnApiResult {
//     message: String,
// }

async fn get_vpnapi_result(ip: &String, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut url: String = "https://vpnapi.io/api/".to_string();
    url.push_str(ip);
    url.push_str("?key=");
    url.push_str(key);


    let body = client.get(url).send()
        .await?
        .text()
        .await?;

    Ok(body)
}

fn get_api_key(k: &String) -> &str {
    k
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