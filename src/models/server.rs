use std::net::IpAddr;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Server {
    // can be either ipv4 or ipv6
    pub ip: IpAddr,
    pub port: u16,
}
