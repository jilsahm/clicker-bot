use std::{path::PathBuf, time::SystemTime};

use clap::Clap;

#[derive(Clap)]
#[clap(
    about = "CLI tool for running a simple clicker bot",
    author = clap::crate_authors!(),
    name = clap::crate_name!(),
    version = clap::crate_version!(),
)]
pub struct Configuration {

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {

    #[clap(about = "running a click storm at the cursor position")]
    Click,
    
    Record(RecordCommand),

    #[clap(about = "replays a given script")]
    Replay,
}

#[derive(Debug, Clap)]
#[clap(about = "record mouse positions by clicking")]
pub struct RecordCommand {

    #[clap(
        about = "the of the file the records will be written in",
        short = 'o',
        long,
        validator = is_valid_out_file,
    )]
    out_file: Option<PathBuf>,
}

impl RecordCommand {

    pub fn out_file(&self) -> PathBuf {
        self.out_file
            .as_ref()
            .map_or_else(Self::default_filename, |path| path.clone())
    }

    fn default_filename() -> PathBuf {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
        PathBuf::from(format!("{}.yaml", timestamp.as_secs()))
    }
}

fn is_valid_out_file(s: &str) -> Result<(), String> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Err(format!("out file '{}' is a directory", s))
    } else if path.is_file() {
        Err(format!("out file '{}' already exists", s))
    } else {
        match path.parent() {
            None => Ok(()),
            Some(parent) => {
                match parent.is_dir() || parent.is_relative() {
                    true => Ok(()),
                    false => Err(format!("folder to '{}' does not exist", s)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::is_valid_out_file;

    use super::RecordCommand;

    
    #[test]
    fn valid_out_files() {
        vec![
            "test.yaml",
            "test/test.yaml"
        ]
        .into_iter()
        .map(|path| is_valid_out_file(path))
        .enumerate()
        .for_each(|(case, result)| assert!(result.is_ok(), "case {} - {}", case, result.err().unwrap()));
    }

    #[test]
    fn invalid_out_files() {
        vec![
            "Cargo.toml",
            "test/commands.yaml",
            ".",
            "test",
        ]
        .into_iter()
        .map(|path| is_valid_out_file(path))
        .enumerate()
        .for_each(|(case, result)| assert!(result.is_err(), "case {}", case));
    }

    #[test]
    fn default_filename() {
        let filename = RecordCommand::default_filename();
        let s = filename.to_string_lossy();
        assert!(s.ends_with(".yaml"), "{} does not end with .yaml", filename.to_string_lossy());
        assert!(!s.starts_with(".yaml"), "{} has no epoch seconds", filename.to_string_lossy());
    }
}
