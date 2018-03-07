mod discover;
mod listener;

use super::plugin;

pub struct dlna;

//impl plugin::Plugin for dlna {
//  pub fn new() -> Self {
//   return dlna {};
//  }
//
// pub fn play(&mut self) {}

//pub fn pause(&mut self) {}

//pub fn stop(&mut self) {}

// pub fn short_name(self) -> &'static str {
//    "ishan"
// }
//}

pub fn render() {
    println!("Render Called");
    let v = discover::SSDPDiscover::new(None, |x, y| {});
}
