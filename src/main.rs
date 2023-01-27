#![allow(dead_code,unused_variables,unused_imports)]
#![feature(ip)]
use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr, AddrParseError}, str::FromStr};
use clap::Parser;
use serde::Deserialize;
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
    // println!("result = {:#?}", r);

    let deserialized: VpnApiResult = serde_json::from_str(&r.unwrap()).unwrap();
    println!("result = {:#?}", deserialized);


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

type Other = serde_json::Map<String, serde_json::Value>;

#[derive(Deserialize, Debug)]
struct VpnApiResult {
    ip: Option<String>,
    security: Option<Security>,
    location: Option<Location>,
    network: Option<Network>,
    message: Option<String>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Deserialize, Debug)]
struct Security {
    vpn: bool,
    proxy: bool,
    tor: bool,
    relay: bool
}

#[derive(Deserialize, Debug)]
struct Location {
    city: String,
    region: String,
    country: String,
    continent: String,
    region_code: String,
    continent_code: String,
    latitude: String,
    longitude: String,
    time_zone: String,
    locale_code: String,
    metro_code: String,
    is_in_european_union: String
}

#[derive(Deserialize, Debug)]
struct Network {
    network: String,
    autonomous_system_number: String,
    autonomous_system_organization: String
}


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