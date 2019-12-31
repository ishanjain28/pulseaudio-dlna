extern crate chrono;
extern crate dbus;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use dbus::{BusType, Connection, Message, MessageItem, Props};

mod pulseaudio;
mod plugin;

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
    plugin::dlna::render();
    //    let a = pulseaudio::Modules::get();
    //   println!("{:?}", a);
    pa.connect(&[
        ("NewPlaybackStream", "org.PulseAudio.Core1"),
        ("PlaybackStreamRemoved", "org.PulseAudio.Core1"),
        ("FallbackSinkUpdated", "org.PulseAudio.Core1"),
        ("DeviceUpdated", "org.PulseAudio.Core1.Stream"),
    ]);
}
