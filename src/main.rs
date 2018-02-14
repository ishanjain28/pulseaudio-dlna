extern crate chrono;
extern crate dbus;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod pulseaudio;


fn main() {
    let mods = pulseaudio::Modules::get();

    //    let id = pulseaudio::Modules::load(
    //      "module-null-sink",
    //    &[
    //      ("sink_name", "ishan"),
    //    ("sink_properties", "device.description=\"bt_speaker\""),
    // ],
    //    );

    let mut pa = pulseaudio::Pulseaudio::new();

    pa.connect(&[("ishan", "crash", |x, y, z| {})]);
}
