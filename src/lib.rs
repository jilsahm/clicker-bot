#[macro_use] extern crate log;

use std::sync::mpsc::channel;

pub use configuration::Configuration;
use configuration::SubCommand;
use keyboard::KeyboardMonitor;
use mouse::VirtualMouse;

mod command;
mod configuration;
mod hardware;
mod mouse;
mod keyboard;
mod event;

pub fn run(config: Configuration) {
    match config.subcommand {
        SubCommand::Click => {
            let (tx, rx) = channel();
            KeyboardMonitor::new(tx).start();
            let mouse = VirtualMouse::new(rx);
            mouse.start();
        },
        SubCommand::Record => todo!(),
        SubCommand::Replay => todo!(),
    }    
}
