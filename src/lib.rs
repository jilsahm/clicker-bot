#[macro_use] extern crate log;

use std::sync::mpsc::channel;

pub use config::Configuration;
use config::SubCommand;
use hardware::VirtualMouse;
use replay::{Recorder, Replayer};

use crate::eventgrid::EventGrid;

mod config;
mod hardware;
mod eventgrid;
mod replay;

pub fn run(config: Configuration) {
    let (tx, rx) = channel();
    EventGrid::new(tx).start();
    match config.subcommand {
        SubCommand::Click => {            
            let mouse = VirtualMouse::new(rx);
            mouse.start();
        },
        SubCommand::Record(config) => {
            let mut recorder = Recorder::new(config.out_file(), rx);
            recorder.start();
        },
        SubCommand::Replay(config) => {
            let commands = config.load_replay();
            info!("loaded replay file with '{}' commands", commands.iter_commands().count());
            let mut replayer = Replayer::new(commands, rx);
            replayer.start();
        },
    }    
}
