fn main() {
    windows::build!(
        Windows::Win32::Foundation::POINT,
        Windows::Win32::UI::KeyboardAndMouseInput::{
            GetAsyncKeyState,
            GetKeyState,
            GetKeyboardState,
            INPUT,
            INPUT_TYPE,
            MOUSE_EVENT_FLAGS,
            MOUSEINPUT,
            KEYBDINPUT,
            KEYBD_EVENT_FLAGS,
            SendInput,
        },
        Windows::Win32::UI::WindowsAndMessaging::GetCursorPos,
    );
}
