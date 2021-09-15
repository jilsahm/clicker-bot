use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Commands {
    loops: String,
    commands: Vec<Command>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {

    #[serde(rename = "mouse")]
    MouseCommand { key: String, x: i32, y: i32, },

    #[serde(rename = "keyboard")]
    KeyboardCommand { key: String, },

    #[serde(rename = "sleep")]
    SleepCommand { millis: u32, },
}

#[cfg(test)]
mod tests {
    use super::Commands;


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
}
