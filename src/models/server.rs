use std::net::IpAddr;

#[derive(Debug, FromForm)]
pub struct Server {
    // can be either ipv4 or ipv6
    pub ip: IpAddr,
    pub port: u16,
}
