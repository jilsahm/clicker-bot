fn main() {
    windows::build!(
        Windows::Win32::Foundation::POINT,
        Windows::Win32::UI::KeyboardAndMouseInput::{
            GetAsyncKeyState,
            INPUT,
            INPUT_TYPE,
            MOUSE_EVENT_FLAGS,
            MOUSEINPUT,
            SendInput,
        },
        Windows::Win32::UI::WindowsAndMessaging::GetCursorPos,
    );
}