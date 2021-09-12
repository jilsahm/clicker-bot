use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::Receiver}, thread, time::Duration};

use bindings::Windows::Win32::{Foundation::{BOOL, POINT}, UI::{KeyboardAndMouseInput::*, WindowsAndMessaging::GetCursorPos}};

use crate::event::Event;

pub enum InputType {
    Mouse,
    Keyboard,
    Hardware,
}

impl Into<INPUT_TYPE> for InputType {

    fn into(self) -> INPUT_TYPE {
        INPUT_TYPE(self as u32)
    }
}

#[derive(Clone, Copy)]
pub enum MouseEvent {
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

impl MouseEvent {

    pub fn of(events: &[Self]) -> MOUSE_EVENT_FLAGS {
        MOUSE_EVENT_FLAGS(events
            .into_iter()
            .fold(0u32, |acc, &x| acc | x as u32))
    }
}

pub struct VirtualMouse {
    rx: Receiver<Event>,
    running: Arc<AtomicBool>,
    fire: Arc<AtomicBool>,
}

impl VirtualMouse {

    pub fn new(rx: Receiver<Event>) -> Self {
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
                Ok(Event::Shutdown) => {
                    info!("receiver shutdown signal");
                    self.running.store(false, Ordering::Relaxed);
                }
                Ok(Event::Signal) => {
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
                    match Self::current_position() {
                        Ok(point) => Self::click(point),
                        Err(_) => warn!("failed to retrieve mouse position"),
                    }                    
                }
                thread::sleep(Duration::from_millis(1));
            }
            info!("shutting down clicker worker");
        });
    }

    fn current_position() -> Result<POINT, ()> {
        unsafe{
            let mut point = POINT { x: 0, y: 0 };
            match GetCursorPos(&mut point) {
                BOOL(0) => Err(()),
                BOOL(_) => Ok(point),
            }
        }
    }

    fn click(position: POINT) {
        unsafe {
            let mouse_input = MOUSEINPUT {
                dx: position.x,
                dy: position.y,
                mouseData: 0,
                dwFlags: MouseEvent::of(&[MouseEvent::Absolute, MouseEvent::VirtualDesk, MouseEvent::LeftDown]),
                time: 0,
                dwExtraInfo: 0,
            };
            let input = &mut INPUT {
                r#type: InputType::Mouse.into(),
                Anonymous: INPUT_0 { mi: mouse_input },
            };
            SendInput(1, input, std::mem::size_of::<INPUT>() as i32);
    
            let mouse_input = MOUSEINPUT {
                dx: position.x,
                dy: position.y,
                mouseData: 0,
                dwFlags: MouseEvent::of(&[MouseEvent::Absolute, MouseEvent::VirtualDesk, MouseEvent::LeftUp]),
                time: 0,
                dwExtraInfo: 0,
            };
            let input = &mut INPUT {
                r#type: InputType::Mouse.into(),
                Anonymous: INPUT_0 { mi: mouse_input },
            };
            SendInput(1, input, std::mem::size_of::<INPUT>() as i32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MouseEvent;

    #[test]
    fn of() {
        vec![
            (0x0000, MouseEvent::of(&[])),
            (0x8000, MouseEvent::of(&[MouseEvent::Absolute])),
            (0x8000, MouseEvent::of(&[MouseEvent::Absolute, MouseEvent::Absolute])),
            (0x8001, MouseEvent::of(&[MouseEvent::Absolute, MouseEvent::Move])),
        ]
        .into_iter()
        .for_each(|(expected, flags)| assert_eq!(expected, flags.0));
    }
}
