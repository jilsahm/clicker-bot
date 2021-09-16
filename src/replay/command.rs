use std::slice::Iter;

use serde::{Deserialize, Serialize};

use crate::hardware::Key;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Loops {
    Infinite,
    Amount(usize),
}

impl Loops {

    pub fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        match self {
            Loops::Infinite => Box::new(0..),
            Loops::Amount(i) => Box::new(0..*i),
        }      
    }
}

impl Default for Loops {

    fn default() -> Self {
        Self::Amount(1)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commands {
    loops: Loops,
    commands: Vec<Command>,
}

impl Commands {

    pub fn iter_loop(&self) -> Box<dyn Iterator<Item = usize>> {
        self.loops.iter()     
    }

    pub fn iter_commands(&self) -> Iter<Command> {
        self.commands.iter()
    }
}

impl From<&mut Vec<Command>> for Commands {

    fn from(commands: &mut Vec<Command>) -> Self {
        Self {
            loops: Loops::Amount(1),
            commands: commands.drain(0..).collect()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Command {

    #[serde(rename = "mouse")]
    MouseCommand { 
        key: Key,
        #[serde(default)]
        loops: Loops, 
        x: i32, 
        y: i32, 
    },

    #[serde(rename = "keyboard")]
    KeyboardCommand { 
        key: Key,
        #[serde(default)]
        loops: Loops, 
    },

    #[serde(rename = "sleep")]
    SleepCommand { millis: u64, },
}

impl Command {

    pub fn is_valid(&self) -> bool {
        match self {
            Self::MouseCommand{ key, .. } => key.is_mouse(),
            Self::KeyboardCommand{ key, .. } => !key.is_mouse(),
            Self::SleepCommand{ .. } => true,
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        match self {
            Command::MouseCommand { loops, .. } => loops.iter(),
            Command::KeyboardCommand { loops, .. } => loops.iter(),
            Command::SleepCommand { .. } => Loops::default().iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{hardware::Key, replay::Command};

    use super::{Commands, Loops};

    fn sample_yaml() -> String {
        String::from_utf8_lossy(&std::fs::read("./test/commands.yaml").expect("valid yaml file")).into()
    }

    #[test]
    fn deserialization() {
        let yaml = sample_yaml();
        let result: Result<Commands, _> = serde_yaml::from_str(&yaml);
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert_eq!(3, result.unwrap().commands.len());
    }

    #[test]
    fn is_valid() {
        vec![
            (true, Command::MouseCommand { key: Key::MouseLeft, loops: Loops::Infinite, x: 0, y: 0, }),
            (false, Command::MouseCommand { key: Key::A, loops: Loops::Infinite, x: 0, y: 0, }),
            (true, Command::KeyboardCommand { key: Key::A, loops: Loops::Infinite, }),
            (false, Command::KeyboardCommand { key: Key::MouseMiddle, loops: Loops::Infinite, }),
            (true, Command::SleepCommand { millis: 0 }),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(case, (expected, sample))| assert_eq!(expected, sample.is_valid(), "case {}", case));
    }

    #[test]
    fn iter_loop_infinite() {
        (10..100)
            .into_iter()
            .map(|i| (i, Commands { loops: Loops::Infinite, commands: Vec::default() }.iter_loop()))
            .for_each(|(cap, iter)| {
                assert!(iter.skip(cap).next().is_some());
            });
    }

    #[test]
    fn iter_loop_amount() {
        (10..100)
            .into_iter()
            .map(|i| (i, Commands { loops: Loops::Amount(i), commands: Vec::default() }.iter_loop()))
            .for_each(|(cap, iter)| {
                assert!(iter.skip(cap).next().is_none());
            });
    }

    #[test]
    fn from() {
        let mut records = vec![
            Command::MouseCommand { key: Key::MouseLeft, loops: Loops::Infinite, x: 0, y: 0, },
            Command::SleepCommand { millis: 0, },
        ];
        let result: Commands = Commands::from(&mut records);
        assert!(records.is_empty());
        assert_eq!(2, result.commands.len());
    }
}
