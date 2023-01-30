#![allow(dead_code,unused_variables,unused_imports)]
#![feature(ip)]
use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr, AddrParseError}, str::FromStr};
use clap::Parser;
use serde::{Serialize, Deserialize};


#[derive(Parser,Default,Debug)]
struct Arguments {
    ip: String,

    /// vpnapi.io key of your account
    #[clap(default_value = "",short, long)]
    key: String,

    /// Reads a list of IPs 
    #[clap(short, long)]
    file: Option<String>,

    /// Saves the API key so you don't need to enter every time
    #[clap(short, long)]
    config: Option<String>,

    /// Pretty prints the results
    #[clap(short, long)]
    pprint: bool,
}

#[derive(Deserialize,Serialize, Debug)]
struct VpnApiResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Network>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    
    #[serde(flatten)]
    other: Other,
}

#[derive(Deserialize,Serialize, Debug)]
struct Security {
    vpn: Option<bool>,
    proxy: Option<bool>,
    tor: Option<bool>,
    relay: Option<bool>
}

#[derive(Deserialize, Serialize, Debug)]
struct Location {
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    continent: Option<String>,
    region_code: Option<String>,
    continent_code: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    time_zone: Option<String>,
    locale_code: Option<String>,
    metro_code: Option<String>,
    is_in_european_union: Option<bool>
}

#[derive(Deserialize, Serialize, Debug)]
struct Network {
    network: Option<String>,
    autonomous_system_number: Option<String>,
    autonomous_system_organization: Option<String>
}

type Other = serde_json::Map<String, serde_json::Value>;

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
        println!("{}", "You need to enter a key".to_string());
        std::process::exit(0)
    }

    let r = get_vpnapi_result(&args.ip, &key).await;

    let deserialized: VpnApiResult = serde_json::from_str(&r.unwrap()).unwrap();


    // Indent if true
    match args.pprint {
        true => println!("{}", serde_json::to_string_pretty(&deserialized).unwrap()),
        false => println!("{}", serde_json::to_string(&deserialized).unwrap())
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

fn set_api_key(k: &String) {
    println!("{}", k)
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