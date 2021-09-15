use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::Receiver}, thread, time::Duration};

use bindings::Windows::Win32::Foundation::POINT;

use crate::{eventgrid::Signal, hardware::Mouse};

pub struct VirtualMouse {
    rx: Receiver<Signal>,
    running: Arc<AtomicBool>,
    fire: Arc<AtomicBool>,
}

impl VirtualMouse {

    pub fn new(rx: Receiver<Signal>) -> Self {
        Self {
            rx,
            running: Arc::new(AtomicBool::new(true)),
            fire: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        info!("stating virtual mouse");
        self.worker();
        while self.running.load(Ordering::Relaxed) {
            match self.rx.recv() {
                Err(_) => {
                    error!("corrupted receiver");
                    self.running.store(false, Ordering::Relaxed);
                }
                Ok(Signal::Shutdown) => {
                    info!("receiver shutdown signal");
                    self.running.store(false, Ordering::Relaxed);
                }
                Ok(Signal::Pause) => {
                    match self.fire.load(Ordering::Acquire) {
                        true => {
                            self.fire.store(false, Ordering::Release);
                            info!("going idle");
                        }
                        false => {
                            self.fire.store(true, Ordering::Release);
                            info!("going fire mode");
                        }
                    }
                }
            }
        }
        info!("shutting down virtual mouse");
    }

    fn worker(&self) {
        let running = self.running.clone();
        let fire = self.fire.clone();
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                if fire.load(Ordering::Relaxed) {
                    match Mouse::current_position() {
                        Ok(point) => Self::click(point),
                        Err(_) => warn!("failed to retrieve mouse position"),
                    }                    
                }
                thread::sleep(Duration::from_millis(1));
            }
            info!("shutting down clicker worker");
        });
    }

    fn click(position: POINT) {
        Mouse::LeftDown.trigger(&position);
        Mouse::LeftUp.trigger(&position);
    }
}
