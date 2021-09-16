use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::Receiver}, thread, time::Duration};

use bindings::Windows::Win32::{Foundation::{BOOL, POINT}, UI::{KeyboardAndMouseInput::{INPUT, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS, SendInput}, WindowsAndMessaging::GetCursorPos}};

use crate::eventgrid::Signal;

use super::InputType;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Mouse {
    Move = 0x0001,
    LeftDown = 0x0002,
    LeftUp = 0x0004,
    RightDown = 0x0008,
    RightUp = 0x0010,
    MiddleDown = 0x0020,
    MiddleUp = 0x0040,
    XDown = 0x0080,
    XUp = 0x00100,
    Wheel = 0x0800,
    HWheel = 0x1000,
    MoveNoCoalesce = 0x2000,
    VirtualDesk = 0x4000,
    Absolute = 0x8000,
}

impl Mouse {

    fn of(events: &[Self]) -> MOUSE_EVENT_FLAGS {
        MOUSE_EVENT_FLAGS(events
            .into_iter()
            .fold(0u32, |acc, &x| acc | x as u32))
    }

    pub fn current_position() -> Result<POINT, ()> {
        unsafe{
            let mut point = POINT { x: 0, y: 0 };
            match GetCursorPos(&mut point) {
                BOOL(0) => Err(()),
                BOOL(_) => Ok(point),
            }
        }
    }

    pub fn trigger(&self, position: &POINT) {
        let input = &mut self.prepare_input(position);
        unsafe { SendInput(1, input, std::mem::size_of::<INPUT>() as i32); }
    }

    fn prepare_input(&self, position: &POINT) -> INPUT {
        let mouse_input = MOUSEINPUT {
            dx: position.x,
            dy: position.y,
            mouseData: 0,
            dwFlags: Mouse::of(&[Mouse::Absolute, Mouse::VirtualDesk, *self]),
            time: 0,
            dwExtraInfo: 0,
        };
        let input = INPUT {
            r#type: InputType::Mouse.into(),
            Anonymous: INPUT_0 { mi: mouse_input },
        };
        input
    }
}

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
        info!("stating virtual mouse, pause/unpause with UP");
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
                            info!("going pause");
                        }
                        false => {
                            self.fire.store(true, Ordering::Release);
                            info!("going unpause");
                        }
                    }
                }
                _ => (),
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

#[cfg(test)]
mod tests {
    use crate::hardware::Mouse;

    #[test]
    fn of() {
        vec![
            (0x0000, Mouse::of(&[])),
            (0x8000, Mouse::of(&[Mouse::Absolute])),
            (0x8000, Mouse::of(&[Mouse::Absolute, Mouse::Absolute])),
            (0x8001, Mouse::of(&[Mouse::Absolute, Mouse::Move])),
        ]
        .into_iter()
        .for_each(|(expected, flags)| assert_eq!(expected, flags.0));
    }
}
