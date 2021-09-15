#[macro_use] extern crate log;

use std::{sync::mpsc::channel, time::Duration};

pub use config::Configuration;
use config::SubCommand;
use mouse::VirtualMouse;
use replay::Recorder;

use crate::{eventgrid::EventGrid, hardware::Key};

mod config;
mod hardware;
mod mouse;
mod eventgrid;
mod replay;

pub fn run(config: Configuration) {
    match config.subcommand {
        SubCommand::Click => {
            let (tx, rx) = channel();
            EventGrid::new(tx).start();
            let mouse = VirtualMouse::new(rx);
            mouse.start();
        },
        SubCommand::Record(config) => {
            let (tx, rx) = channel();
            EventGrid::new(tx).start();
            let mut recorder = Recorder::new(config.out_file(), rx);
            recorder.start();
        },
        SubCommand::Replay => {
            std::thread::sleep(Duration::from_secs(5));
            Key::A.press();
            Key::A.release();
        },
    }    
}
