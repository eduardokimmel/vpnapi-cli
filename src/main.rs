// #![allow(dead_code,unused_variables,unused_imports)]
#![feature(ip)]
use std::{net::IpAddr, str::FromStr, io, fs::File, path::Path};
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use dirs::data_dir;


#[derive(Parser,Default,Debug)]
struct Arguments {
    ip: String,

    /// vpnapi.io key of your account
    #[clap(default_value = "",short, long)]
    key: String,

    /// Reads a list of IPs - Not implemented
    // #[clap(short, long)]
    // file: Option<String>,

    /// Pretty prints the result
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

    // if main arg is "config", asks for a key and saves it to a file
    if &args.ip == "config" {
        match set_api_key() {
            Ok(_) => println!("Saved new key"),
            Err(_) => println!("Faile to write to file")
        }
        std::process::exit(0)
    }

    // Check if makes sense to query for the IP
    if !check_if_valid_ip(&args.ip) {
        println!("Invalid IP");
        std::process::exit(0)
    }

    if !check_if_global_ip(&args.ip) {
        println!("Not global IP");
        std::process::exit(0)
    }

    // If -k is null or "", try to get the key from the secret file
    let key = get_api_key(&args.key);
    match key {
        Err(_) => {
            println!("Unable to read key file, try running `vpnapi-cli config` or entering `-k <your key>`");
            std::process::exit(0)
        },
        _ => ()
    }
    let key = key.unwrap();
    
    // If the key is "", asks for a key
    if key == "" {
        println!("You need to enter or set a key");
        std::process::exit(0)
    }

    // Makes the request
    let r = get_vpnapi_result(&args.ip, &key).await;

    // Turns it into a serde_json
    let deserialized: VpnApiResult = serde_json::from_str(&r.unwrap()).unwrap();

    // If any message, prints it and quit
    match &deserialized.message {
        Some(m) => {println!("{}", m); std::process::exit(0)},
        _ => ()
    }

    // Indent if --pprint == true
    match args.pprint {
        true => println!("{}", serde_json::to_string_pretty(&deserialized).unwrap()),
        false => println!("{}", serde_json::to_string(&deserialized).unwrap())
    }

}

fn check_if_valid_ip(s: &String) -> bool {
    let ip = IpAddr::from_str(&s);
    match ip {
        Ok(_) => true,
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

fn set_api_key() -> std::io::Result<()> {
    println!("Enter your VPNAPI key:");

    let data_dir = data_dir().expect("Not found").display().to_string();
    let key_file = Path::new(&data_dir).join("vpnapi");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut file = File::create(key_file)?;
    file.write(input.trim().as_bytes())?;
    Ok(())
}

fn get_api_key(k: &String) -> Result<String, io::Error> {
    if k != "" {
        return Ok(k.to_string());
    } 

    let data_dir = dirs::data_dir().expect("Not found").display().to_string();
    let key_file = Path::new(&data_dir).join("vpnapi");

    let mut file = File::open(key_file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(data)
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