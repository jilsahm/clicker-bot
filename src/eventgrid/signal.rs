use crate::hardware::Key;

#[derive(Clone, Copy, Debug)]
pub enum Signal {
    Input(Key),
    Pause,
    Shutdown,
}

impl From<Key> for Signal {

    fn from(key: Key) -> Self {
        match key {
            Key::KeyboardUp => Self::Pause,
            Key::KeyboardDown => Self::Shutdown,
            _ => Self::Input(key),
        }
    }
}
