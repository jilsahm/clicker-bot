use bindings::Windows::Win32::UI::KeyboardAndMouseInput::INPUT_TYPE;

mod keyboard;
mod mouse;

pub use keyboard::Key;
pub use mouse::{Mouse, VirtualMouse};

#[allow(dead_code)]
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
