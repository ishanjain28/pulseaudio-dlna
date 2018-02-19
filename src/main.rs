extern crate chrono;
extern crate dbus;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use dbus::{BusType, Connection, Message, MessageItem, Props};
use std::error::Error;

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

    let x = |x: Result<&Message, Error>| {};
    let y = |x: Result<&Message, Error>| {};

    let w = |x: Result<&Message, Error>| {};

    let z = |x: Result<&Message, Error>| {};

    //    pa.connect(&[
    //      ("NewPlaybackStream", "org.PulseAudio.Core1", x),
    //   ("PlaybackStreamRemoved", "org.PulseAudio.Core1", y),
    //   ("FallbackSinkUpdated", "org.PulseAudio.Core1", w),
    //  ("DeviceUpdated", "org.PulseAudio.Core1", z),
    //  ]);
}
