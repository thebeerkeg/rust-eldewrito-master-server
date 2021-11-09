use crate::models::server::Server;
use std::time::SystemTime;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Announce {
    pub server: Server,
    pub socket_addr: Option<SocketAddr>,
    pub timestamp: SystemTime,
}

impl Announce {
    pub fn server_addr(&self) -> String {
        let ip = match self.socket_addr {
            None => self.server.ip,
            Some(v) => v.ip()
        };

        format!("{}:{}", ip, self.server.port)
    }
}
