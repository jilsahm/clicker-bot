#[macro_use] extern crate log;

use std::{sync::mpsc::channel, time::Duration};

pub use configuration::Configuration;
use configuration::SubCommand;
use mouse::VirtualMouse;

use crate::{eventgrid::EventGrid, hardware::Key};

mod command;
mod configuration;
mod hardware;
mod mouse;
mod eventgrid;

pub fn run(config: Configuration) {
    match config.subcommand {
        SubCommand::Click => {
            let (tx, rx) = channel();
            EventGrid::new(tx).start();
            let mouse = VirtualMouse::new(rx);
            mouse.start();
        },
        SubCommand::Record => todo!(),
        SubCommand::Replay => {
            std::thread::sleep(Duration::from_secs(5));
            Key::A.press();
            Key::A.release();
        },
    }    
}
