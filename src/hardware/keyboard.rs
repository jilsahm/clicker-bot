use bindings::Windows::Win32::UI::KeyboardAndMouseInput::{INPUT, INPUT_0, KEYBDINPUT, KEYBD_EVENT_FLAGS, SendInput};

use super::InputType;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left = 0x01,
    Right = 0x03,
    Up = 0x26,
    Down = 0x28,
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

impl Key {

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
