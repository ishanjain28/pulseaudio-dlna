use std::process::{Command, ExitStatus};
use std::{env, fs, path, process, str, string};
use regex::Regex;
use dbus::{arg, BusType, Connection, Message, MessageItem, MessageItemArray, Props, Signature};
pub struct Pulseaudio<'a> {
    streams: Vec<String>,
    sinks: Option<Props<'a>>,
    fallback_sink: Option<Props<'a>>,
    system_sinks: Vec<String>,
    bus: Option<Connection>,
}

pub struct Modules;

pub struct PulseWatcher;
pub struct PulseSink;

const MODULE_NULL_SINK: &str = "module-null-sink";
const MODULE_DBUS_PROTOCOL: &str = "module-dbus-protocol";

impl Modules {
    pub fn get() -> Option<Vec<string::String>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)\s+([\w-]+)(.*?)\n").unwrap();
        }

        let output = match Command::new("pactl")
            .arg("list")
            .arg("modules")
            .arg("short")
            .output()
        {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };

        let s = str::from_utf8(&output.stdout).unwrap();
        let modules = RE.captures_iter(s)
            .map(|x| x.get(2).unwrap().as_str())
            .map(|x| x.to_owned())
            .collect();

        Some(modules)
    }

    pub fn load(mod_name: &str, mods_param: &[(&str, &str)]) -> Option<u32> {
        let mut args: Vec<String> = Vec::new();

        for m in mods_param {
            args.push(format!("{}={}", m.0, m.1));
        }

        let output = match Command::new("pactl")
            .arg("load-module")
            .arg(mod_name)
            .args(args)
            .output()
        {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };

        let output = str::from_utf8(&output.stdout).unwrap().trim_right();

        match output.parse::<u32>() {
            Ok(v) => Some(v),
            Err(e) => None,
        }
    }

    pub fn unload(module: u32) -> ExitStatus {
        Command::new("pactl")
            .arg("unload-module")
            .arg(module.to_string())
            .status()
            .expect(&format!(
                "failed to unload module {} (\"pactl unload-module {}\")",
                module, module
            ))
    }
}

impl<'a> Pulseaudio<'a> {
    pub fn new() -> Pulseaudio<'static> {
        Pulseaudio {
            streams: vec![String::new()],
            sinks: None,
            fallback_sink: None,
            system_sinks: vec![String::new()],
            bus: None,
        }
    }

    fn get_bus(&mut self) -> Result<Connection, &'static str> {
        let mods = match Modules::get() {
            Some(v) => v,
            None => {
                return Err("error in fetching modules");
            }
        };

        match mods.into_iter().find(|x| x == MODULE_DBUS_PROTOCOL) {
            Some(v) => {
                println!("{} is already loaded", MODULE_DBUS_PROTOCOL);
            }
            None => match Modules::load(MODULE_DBUS_PROTOCOL, &[]) {
                Some(v) => println!("loaded {}({})", MODULE_DBUS_PROTOCOL, v),
                None => {
                    return Err("failed to load module-dbus-protocol");
                }
            },
        }

        // Get Pulseaudio DBUS urls using different methods.
        let bus_addr = self.get_bus_addresses();

        // Try connecting to pulse audio Dbus using one of the available url
        for bus in bus_addr {
            println!("Connecting to pulseaudio on {}", bus);

            let conn = match Connection::open_private(&bus) {
                Ok(v) => {
                    println!("Connected to pulseaudio at {}", bus);
                    return Ok(v);
                }
                Err(e) => {
                    println!("error in connecting to pulseaudio at {}", e);
                    continue;
                }
            };
        }

        Err("failed to connect to any available dbus addresses")
    }

    fn get_bus_addresses(&mut self) -> Vec<String> {
        let mut bus_addresses: Vec<String> = Vec::new();

        // Probe PULSE_DBUS_SERVER
        match env::var("PULSE_DBUS_SERVER") {
            Ok(v) => for addr in v.split(";") {
                bus_addresses.push(addr.to_owned());
            },
            Err(e) => println!("error in probing $PULSE_DBUS_SERVER: {}", e),
        };

        // Probe /run/pulse/dbus-socket
        match fs::File::open("/run/pulse/dbus-socket") {
            Ok(v) => {
                bus_addresses.push("unix:path=/run/pulse/dbus-socket".to_owned());
            }
            Err(e) => {
                println!("error in probing /run/pulse/dbus-socket {}", e);
            }
        };

        // Probe XDG_RUNTIME_DIR
        match env::var("XDG_RUNTIME_DIR") {
            Ok(v) => {
                let p = path::Path::new(&v).join("pulse/dbus-socket");

                match fs::File::open(&p) {
                    Ok(v) => {
                        bus_addresses.push(format!("unix:path={}", p.to_str().unwrap()));
                    }
                    Err(e) => {
                        println!("error in probing $XDG_RUNTIME_DIR {}", e);
                    }
                };
            }
            Err(e) => {
                println!("failed to probe $XDG_RUNTIME_DIR {}", e);
            }
        }

        // dbus_server_lookup
        let addr = self.dbus_server_lookup();
        match addr {
            Ok(v) => {
                bus_addresses.push(v);
            }
            Err(e) => {
                println!("failed in dbus_server_lookup: {}", e);
            }
        };

        // Remove duplicate items from bus_addresses
        // There'll probably never be a lot of items in this vector
        // So, It's okay to uses sort and dedup like this to remove all dups
        bus_addresses.sort();
        bus_addresses.dedup();

        bus_addresses
    }

    fn dbus_server_lookup(&mut self) -> Result<String, String> {
        let conn = Connection::get_private(BusType::Session).unwrap();

        let prop = Props::new(
            &conn,
            "org.PulseAudio1",
            "/org/pulseaudio/server_lookup1",
            "org.PulseAudio.ServerLookup1",
            20,
        );
        match prop.get("Address") {
            Ok(v) => Ok(v.inner::<&str>().unwrap().to_owned()),
            Err(e) => Err(e.to_string()),
        }
    }

    // signals signature: signal_name, interface, signal_handler
    pub fn connect(&mut self, signals: &[(&str, &str)]) {
        let bus = match self.get_bus() {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };

        let core = Props::new(
            &bus,
            "org.PulseAudio.Core1",
            "/org/pulseaudio/core1",
            "org.PulseAudio.Core1",
            20,
        );
        //        println!("CORE_ALL: {:?}", core.get_all());

        for signal in signals {
            let msg =
                Message::new_method_call(signal.1, "/org/pulseaudio/core1", signal.1, signal.0)
                    .unwrap();
            bus.add_handler(
                bus.send_with_reply(msg, move |reply| println!("REPLY {:?}", reply))
                    .unwrap(),
            );
            let full_path = format!("{}.{}", signal.1, signal.0);
            println!("{}", full_path);
        }

        let fallback_sink_path = core.get("FallbackSink");
        let system_sink_paths = core.get("Sinks");

        //        println!(
        //            "FALLBACK_SINK_PATH: {:?} SYSTEM_SINK_PATHS: {:?}",
        //            fallback_sink_path, system_sink_paths
        //        );
        //        PulseSink::new(&bus, fallback_sink_path.unwrap()
        //
    }

    pub fn update_sinks(&mut self) {}

    pub fn create_sink(&mut self, name: &str, desc: &str) {
        let mod_id = Modules::load(
            MODULE_NULL_SINK,
            &[
                ("sink_name", name),
                (
                    "sink_properties",
                    &format!("device.description={}", desc.replace(" ", "\\ ")),
                ),
            ],
        );

        if mod_id.unwrap() > 0 {
            self.update_sinks();
        }
    }

    pub fn delete_sink(mod_id: u32) -> ExitStatus {
        Modules::unload(mod_id)
    }
}

impl PulseSink {
    fn new(bus: &Connection, sink_path: MessageItem) {
        let core = Props::new(
            &bus,
            "org.Pulseaudio.Core1.Device",
            sink_path.inner::<&str>().unwrap(),
            "org.Pulseaudio.Core1.Device",
            20,
        );

        let device = core.get("PropertyList");
        let props_list = core.get("OwnerModule");

        println!("{:?} {:?}", device, props_list);
    }
}
