use std::{convert::TryFrom, sync::mpsc::Sender, thread, time::Duration};

use bindings::Windows::Win32::{Foundation::BOOL, UI::KeyboardAndMouseInput::{GetKeyState, GetKeyboardState}};

use crate::hardware::Key;

pub use self::signal::Signal;

mod signal;

pub struct EventGrid {
    memory: [bool; 256],
    tx: Sender<Signal>,
}

impl EventGrid {

    pub fn new(tx: Sender<Signal>) -> Self {
        Self {
            memory: [false; 256],
            tx,
        }
    }

    pub fn start(mut self) {
        thread::spawn(move || unsafe {
            let inputs = &mut [0u8; 256];
            loop {
                let _ = GetKeyState(0);               
                match GetKeyboardState(inputs.as_mut_ptr()) {
                    BOOL(0) => error!("failed to retrieve keyboard state"),
                    BOOL(_) => 
                        inputs
                            .iter()
                            .enumerate()
                            .map(|(key, state)| (Key::try_from(key as u8), state))
                            .filter(|(key, _)| key.is_ok())
                            .map(|(key, state)| (key.unwrap(), state))
                            .for_each(|(key, &state)| {
                                if state > 1 && !self.memory[key as usize] {
                                    let signal = key.into();
                                    match self.tx.send(signal) {
                                        Ok(_) => info!("published {:?} on eventgrid", signal),
                                        Err(_) => return error!("broken signal sender, prepare shutdown..."),
                                    }
                                    self.memory[key as usize] = true;
                                } else if state <= 1 {
                                    self.memory[key as usize] = false;
                                }
                            })
                }
                thread::sleep(Duration::from_millis(20));
            }
        });
    }
}
