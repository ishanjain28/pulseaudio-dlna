use std::net::Ipv4Addr;

pub struct SSDPDiscover {
    address: Ipv4Addr,
    host: Option<Ipv4Addr>,
    port: u8,
    mx: u8,
    ttl: u8,
    amount: u8,
}

impl SSDPDiscover {
    pub fn new<F>(host: Option<Ipv4Addr>, cb: F) -> SSDPDiscover
    where
        F: FnOnce(Ipv4Addr, Ipv4Addr),
    {
        SSDPDiscover {
            address: Ipv4Addr::new(239, 255, 255, 250),
            host: host,
            port: 1900,
            mx: 3,
            ttl: 10,
            amount: 5,
        }
    }
    fn refresh_address(self) {}
}

pub fn discover() {
    println!("Discovery Started");
}
