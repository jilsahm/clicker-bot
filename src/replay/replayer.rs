use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::Receiver}, thread, time::Duration};

use bindings::Windows::Win32::Foundation::POINT;

use crate::{eventgrid::Signal, hardware::{Key, Mouse}};

use super::Commands;

pub struct Replayer {
    commands: Commands,
    rx: Receiver<Signal>,
    paused: Arc<AtomicBool>,
}

impl Replayer {

    pub fn new(commands: Commands, rx: Receiver<Signal>) -> Self {
        Self {
            commands,
            rx,
            paused: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start(&mut self) {
        self.worker();
        info!("replay prepared, start/pause with UP");
        while let Ok(event) = self.rx.recv() {
            match event {
                Signal::Input(_) => (),
                Signal::Pause => self.paused.store(self.paused.load(Ordering::Acquire), Ordering::Release),
                Signal::Shutdown => break,
            }
        }
        info!("shutting down replayer")
    }

    fn worker(&self) {
        let paused = self.paused.clone();
        let commands = self.commands.clone();
        thread::spawn(move || {
            commands
                .iter_loop()
                .flat_map(|_| commands.iter_commands())
                .for_each(|command| {
                    info!("replaying {:?}", command);
                    for _i in command.iter() {
                        while paused.load(Ordering::Relaxed) {
                            thread::sleep(Duration::from_millis(20));
                        }
                        match command {
                            super::Command::MouseCommand { key, x, y , ..} =>  {
                                let point = POINT{ x: *x, y: *y, };
                                match key {
                                    Key::MouseLeft => {
                                        Mouse::LeftDown.trigger(&point);
                                        Mouse::LeftUp.trigger(&point);
                                    },
                                    Key::MouseRight => {
                                        Mouse::RightDown.trigger(&point);
                                        Mouse::RightUp.trigger(&point);
                                    },
                                    _ => error!("faulty/unimplemented mouse command {:?}", key),
                                }
                            },
                            super::Command::KeyboardCommand { key , ..} => {
                                key.press();
                                key.release();
                            },
                            super::Command::SleepCommand { millis } => thread::sleep(Duration::from_millis(*millis)),
                        }
                    }
                });
        });
    }
}
