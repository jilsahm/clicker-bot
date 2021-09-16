use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{INPUT, INPUT_0, KEYBDINPUT, KEYBD_EVENT_FLAGS, SendInput};

use super::InputType;

#[allow(dead_code)]
enum KeyState {
    Pressed = 0x0000,
    Released = 0x0002,
}

impl Into<KEYBD_EVENT_FLAGS> for KeyState {

    fn into(self) -> KEYBD_EVENT_FLAGS {
        KEYBD_EVENT_FLAGS(self as u32)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
pub enum Key {
    MouseLeft = 0x01,
    MouseRight = 0x02,
    MouseMiddle = 0x04,
    MouseX1 = 0x05,
    MouseX2 = 0x06,
    KeyboardLeft = 0x25,    
    KeyboardUp = 0x26,
    KeyboardRight = 0x27,
    KeyboardDown = 0x28,
    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,
    Escape = 0x1B,
}

impl Into<INPUT> for Key {

    fn into(self) -> INPUT {
        let keyboard_input = KEYBDINPUT {
            wVk: self as u16,
            wScan: 0,
            dwFlags: KeyState::Pressed.into(),
            time: 0,
            dwExtraInfo: 0
        };
        let input = INPUT {
            r#type: InputType::Keyboard.into(),
            Anonymous: INPUT_0 { ki: keyboard_input },
        };
        input
    }
}

impl TryFrom<u8> for Key {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0x01 => Ok(Key::MouseLeft),
            0x02 => Ok(Key::MouseRight),
            0x04 => Ok(Key::MouseMiddle),
            0x05 => Ok(Key::MouseX1),
            0x06 => Ok(Key::MouseX2),
            0x25 => Ok(Key::KeyboardLeft),
            0x26 => Ok(Key::KeyboardUp),
            0x27 => Ok(Key::KeyboardRight),
            0x28 => Ok(Key::KeyboardDown),
            0x41 => Ok(Key::A),
            0x42 => Ok(Key::B),
            0x43 => Ok(Key::C),
            0x44 => Ok(Key::D),
            0x45 => Ok(Key::E),
            0x46 => Ok(Key::F),
            0x47 => Ok(Key::G),
            0x48 => Ok(Key::H),
            0x49 => Ok(Key::I),
            0x4A => Ok(Key::J),
            0x4B => Ok(Key::K),
            0x4C => Ok(Key::L),
            0x4D => Ok(Key::M),
            0x4E => Ok(Key::N),
            0x4F => Ok(Key::O),
            0x50 => Ok(Key::P),
            0x51 => Ok(Key::Q),
            0x52 => Ok(Key::R),
            0x53 => Ok(Key::S),
            0x54 => Ok(Key::T),
            0x55 => Ok(Key::U),
            0x56 => Ok(Key::V),
            0x57 => Ok(Key::W),
            0x58 => Ok(Key::X),
            0x59 => Ok(Key::Y),
            0x5A => Ok(Key::Z),
            0x1B => Ok(Key::Escape),
            _ => Err(())
        }
    }
}

impl Key {

    pub fn is_mouse(&self) -> bool {
        match self {
            Key::MouseLeft 
            | Key::MouseRight 
            | Key::MouseMiddle
            | Key::MouseX1
            | Key::MouseX2 => true,
            _ => false,
        }
    }

    pub fn press(&self) {
        let input = &mut self.prepare_input(KeyState::Pressed);
        unsafe { SendInput(1, input, std::mem::size_of::<INPUT>() as i32); }
    }

    pub fn release(&self) {
        let input = &mut self.prepare_input(KeyState::Released);
        unsafe { SendInput(1, input, std::mem::size_of::<INPUT>() as i32); }
    }

    fn prepare_input(&self, key_state: KeyState) -> INPUT {
        let keyboard_input = KEYBDINPUT {
            wVk: *self as u16,
            wScan: 0,
            dwFlags: key_state.into(),
            time: 0,
            dwExtraInfo: 0
        };
        let input = INPUT {
            r#type: InputType::Keyboard.into(),
            Anonymous: INPUT_0 { ki: keyboard_input },
        };
        input
    }
}
