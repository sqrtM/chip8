use display::window;
use std::{
    thread::{self, sleep},
    time::Duration,
};

mod display;
mod internals;
mod utils;

use crate::internals::memory::Ram;

fn main() {
    thread::spawn(|| window::init());

    loop {
        Ram::init();
        sleep(Duration::from_secs(1))
    }

    println!("check");
}
