extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod pulseaudio;


fn main() {
    let mods = pulseaudio::Modules::get();

    println!("{:?}", mods);


    println!("Hello world");
}
