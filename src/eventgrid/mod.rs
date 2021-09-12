use std::{sync::mpsc::Sender, thread, time::Duration};

use bindings::Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState;

use crate::hardware::Key;

pub use self::signal::Signal;

mod signal;

pub struct EventGrid {
    pause: (Key, bool),
    shutdown: Key,
    tx: Sender<Signal>,
}

impl EventGrid {

    pub fn new(tx: Sender<Signal>) -> Self {
        Self {
            pause: (Key::Up, false),
            shutdown: Key::Down,
            tx,
        }
    }

    pub fn start(mut self) {
        thread::spawn(move || unsafe {
            while GetAsyncKeyState(self.shutdown as i32) == 0 {
                let state = GetAsyncKeyState(self.pause.0 as i32);
                if !self.pause.1 && state != 0 {
                    match self.tx.send(Signal::Pause) {
                        Ok(_) => info!("send signal triggered by {:?}", self.pause.0),
                        Err(_) => return error!("signal send failure"),
                    }
                    self.pause.1 = true;
                } else if state == 0 {
                    self.pause.1 = false;
                }
                thread::sleep(Duration::from_millis(20));
            }
            match self.tx.send(Signal::Shutdown) {
                Ok(_) => info!("send shutdown triggered by {:?}", self.shutdown),
                Err(_) => error!("shutdown send failure"),
            }
        });
    }
}
