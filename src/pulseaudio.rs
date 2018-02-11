extern crate lazy_static;
extern crate regex;

use std::process::{Command, ExitStatus};
use std::{str, string};
use regex::Regex;

struct Pulseaudio {
    streams: Vec<String>,
    sinks: Vec<String>,
    fallback_sink: Vec<String>,
    system_sinks: Vec<String>,
}

pub struct Modules;


impl Modules {
    pub fn get() -> Option<Vec<string::String>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)\s+([\w-]+)(.*?)\n").unwrap();
        }

        let output = Command::new("pactl")
            .arg("list")
            .arg("modules")
            .arg("short")
            .output()
            .expect("failed to execute \"pactl list modules short\"");


        let s = str::from_utf8(&output.stdout).unwrap();
        let modules = RE.captures_iter(s)
            .map(|x| x.get(2).unwrap().as_str())
            .map(|x| x.to_owned())
            .collect();

        Some(modules)
    }

    fn load(mod_name: &str) {
        Command::new("pactl").arg("load-module").arg(mod_name)
    }

    fn unload(module: &str) -> ExitStatus {
        Command::new("pactl")
            .arg("unload-module")
            .arg(format!("{}", module))
            .status()
            .expect(&format!(
                "failed to unload module {} (\"pactl unload-module {}\")",
                module,
                module
            ))
    }
}


impl Pulseaudio {
    fn new() -> Pulseaudio {
        Pulseaudio {
            streams: vec![String::from("")],
            sinks: vec![String::from("")],
            fallback_sink: vec![String::from("")],
            system_sinks: vec![String::from("")],
        }
    }

    fn connect(&mut self) {}
}
