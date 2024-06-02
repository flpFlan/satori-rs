use std::net::IpAddr;

#[derive(Clone)]
pub struct NetAPPConfig {
    pub host: IpAddr,
    pub port: u16,
    pub authorize: Option<String>,
}
