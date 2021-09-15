use bindings::Windows::Win32::{Foundation::{BOOL, POINT}, UI::{KeyboardAndMouseInput::{INPUT, INPUT_0, MOUSEINPUT, MOUSE_EVENT_FLAGS, SendInput}, WindowsAndMessaging::GetCursorPos}};

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
