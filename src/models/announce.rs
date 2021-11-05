use crate::models::server::Server;
use std::time::SystemTime;
use std::net::SocketAddr;

pub struct Announce {
    pub server: Server,
    pub timestamp: SystemTime,
    pub remote_ip: SocketAddr,
}

impl Announce {
    pub fn as_addr(&self) -> String {
        format!("{}:{}", self.remote_ip.ip(), self.server.port)
    }
}
