use std::net::Ipv4Addr;

pub trait Plugin {
    fn new() -> Self;

    fn play(&mut self);

    fn pause(&mut self);

    fn stop(&mut self);

    fn short_name(self) -> &'static str;
}
