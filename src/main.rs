use std::collections::HashMap;
use std::net::IpAddr;
use std::net::Ipv4Addr;

use default_net::gateway::get_default_gateway;
use local_ip_address::local_ip;
use ping::ping;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn ip_address() {
    match local_ip() {
        Ok(addresses) => println!("[V] IP Address: {:?}", addresses),
        Err(err) => panic!("[X] IP Addresses: {}", err),
    }
}

fn gateway() -> IpAddr {
    match get_default_gateway() {
        Ok(gateway) => {
            let gateway_addr = gateway.ip_addr;
            println!("[V] Gateway: {}", gateway_addr.to_string());
            gateway_addr
        }
        Err(err) => panic!("[X] Gateway: {}", err),
    }
}

fn ping_addr(addr: &IpAddr) {
    match ping(addr.to_owned(), None, None, None, None, None) {
        Ok(_) => println!("[V] Pinged {}", addr.to_string()),
        Err(err) => panic!("[X] Ping : Could not ping {}, {}", addr.to_string(), err),
    }
}

fn internal_dns() {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = match resolver.lookup_ip("www.google.fr") {
        Ok(response) => response,
        Err(err) => panic!("[X] Internal DNS : Could not perform DNS Query. {}", err),
    };
    match response.iter().next() {
        Some(result) => println!(
            "[V] Internal DNS : www.google.com resolved to {}",
            result.to_string()
        ),
        None => panic!("[X] Internal DNS : Could not find an IP for www.google.fr"),
    };
}

fn external_ip() {
    match reqwest::blocking::get("https://api.ipify.org?format=json") {
        Ok(response) => match response.json::<HashMap<String, String>>() {
            Ok(data) => println!("[V] External IP : {}", data["ip"]),
            Err(err) => panic!("[X] External IP : {}", err),
        },
        Err(err) => panic!("[X] External IP : {}", err),
    }
}

fn main() {
    ip_address();
    let gateway_addr = gateway();
    ping_addr(&gateway_addr);
    ping_addr(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
    internal_dns();
    external_ip();
}
