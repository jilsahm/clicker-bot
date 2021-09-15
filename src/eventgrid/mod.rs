use std::{convert::TryFrom, sync::mpsc::Sender, thread, time::Duration};

use bindings::Windows::Win32::{Foundation::BOOL, UI::KeyboardAndMouseInput::{GetAsyncKeyState, GetKeyState, GetKeyboardState}};

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
            //while GetAsyncKeyState(Key::KeyboardDown as i32) == 0 {
            loop {
                /*let state = GetAsyncKeyState(self.pause.0 as i32);
                if !self.pause.1 && state != 0 {
                    match self.tx.send(Signal::Pause) {
                        Ok(_) => info!("send signal triggered by {:?}", self.pause.0),
                        Err(_) => return error!("signal send failure"),
                    }
                    self.pause.1 = true;
                } else if state == 0 {
                    self.pause.1 = false;
                }
                thread::sleep(Duration::from_millis(20));*/  
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
                                        Ok(_) => info!("publish {:?} on eventgrid", signal),
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
