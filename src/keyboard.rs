use std::{sync::mpsc::Sender, thread};

use bindings::Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState;

use crate::event::Event;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Up = 0x26,
    Down = 0x28,
}

pub struct KeyboardMonitor {
    signal: (Key, bool),
    shutdown: Key,
    tx: Sender<Event>,
}

impl KeyboardMonitor {

    pub fn new(tx: Sender<Event>) -> Self {
        Self {
            signal: (Key::Up, false),
            shutdown: Key::Down,
            tx,
        }
    }

    pub fn start(mut self) {
        thread::spawn(move || unsafe {
            while GetAsyncKeyState(self.shutdown as i32) == 0 {
                let state = GetAsyncKeyState(self.signal.0 as i32);
                if !self.signal.1 && state != 0 {
                    match self.tx.send(Event::Signal) {
                        Ok(_) => info!("send signal triggered by {:?}", self.signal.0),
                        Err(_) => return error!("signal send failure"),
                    }
                    self.signal.1 = true;
                } else if state == 0 {
                    self.signal.1 = false;
                }
                thread::yield_now();
            }
            match self.tx.send(Event::Shutdown) {
                Ok(_) => info!("send shutdown triggered by {:?}", self.shutdown),
                Err(_) => error!("shutdown send failure"),
            }
        });
    }
}