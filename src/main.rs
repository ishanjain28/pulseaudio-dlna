extern crate chrono;
extern crate dbus;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate regex;

mod pulseaudio;


fn main() {
    let mods = pulseaudio::Modules::get();

    println!("{:?}", mods);

    let id = pulseaudio::Modules::load(
        "module-null-sink",
        &[
            ("sink_name", "ishan"),
            ("sink_properties", "device.description=\"bt_speaker\""),
        ],
    );

    println!("{:?}", id);

    let mods = pulseaudio::Modules::get();

    println!("{:?}", mods);

    pulseaudio::Modules::unload(id.unwrap());
    let mods = pulseaudio::Modules::get();

    println!("{:?}", mods);

    println!("Hello world");
}
