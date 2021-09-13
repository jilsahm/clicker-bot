use std::{convert::TryFrom, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

use bindings::Windows::Win32::{Foundation::{BOOL, POINT}, UI::{KeyboardAndMouseInput::{GetKeyState, GetKeyboardState}, WindowsAndMessaging::GetCursorPos}};

use crate::{config::Command, hardware::Key};

pub struct Recorder {
    memory: [bool; 256],
    records: Vec<Command>,
    running: Arc<AtomicBool>,
}

impl Recorder {
    
    pub fn new() -> Self {
        Self { 
            memory: [false; 256],
            records: Vec::with_capacity(128),
            running: Arc::new(AtomicBool::from(true)),
        }
    }

    pub fn start(&mut self) {
        info!("start recording");
        self.worker();
        unsafe {
            let inputs = &mut [0u8; 256];
            while self.running.load(Ordering::Relaxed) {                
                let _ = GetKeyState(0); // Because Pepe
                match GetKeyboardState(inputs.as_mut_ptr()) {
                    BOOL(0) => error!("failed to retrieve keyboard state"),
                    BOOL(_) => 
                        inputs
                            .iter()
                            .enumerate()
                            //.filter(|(key, &state)| state > 0)
                            .map(|(key, state)| (Key::try_from(key as u8), state))
                            .filter(|(key, _)| key.is_ok())
                            .map(|(key, state)| (key.unwrap(), state))
                            .for_each(|(key, &state)| {
                                if state > 1 && !self.memory[key as usize] {
                                    self.record(key);
                                    self.memory[key as usize] = true;
                                } else if state <= 1 {
                                    self.memory[key as usize] = false;
                                }
                            })
                }
                thread::sleep(Duration::from_millis(20));
            }
        }
        info!("recording stopped");
        info!("recorder {:?}", self.records);
    }

    unsafe fn record(&mut self, key: Key) {
        if key.is_mouse() {
            let position= &mut POINT { x: 0, y: 0 };
            let _ = GetCursorPos(position);
            self.records.push(Command::MouseCommand{
                key: format!("{:?}", key),
                x: position.x,
                y: position.y,
            });
            info!("{:?} at ({}|{}) recorded", key, position.x, position.y);
        } else {
            self.records.push(Command::KeyboardCommand{ 
                key: format!("{:?}", key),
            });
            info!("{:?} recorded", key);
        }
    }

    fn worker(&self) {
        let runnint = self.running.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(10));
            info!("shutting down recorder");
            runnint.store(false, Ordering::Relaxed);
        });
    }
}
