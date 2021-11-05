use std::net::IpAddr;

#[derive(FromForm)]
pub struct Announce {
    // can be either ipv4 or ipv6
    ip: IpAddr,
    port: u16,
}

// announcing servers to the server browser
#[get("/announce?<announce..>")]
pub fn announce(announce: Announce) -> String {
    format!("Announcing server: ( ip: {}, port: {} )", announce.ip.to_string(), announce.port)
}
