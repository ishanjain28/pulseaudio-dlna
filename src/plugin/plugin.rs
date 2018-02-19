use std::net::Ipv4Addr;

pub trait Plugin {
    fn play(&mut self);

    fn pause(&mut self);

    fn stop(&mut self);
}
