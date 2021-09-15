use std::{fs::File, io::BufWriter, path::PathBuf, sync::mpsc::Receiver};

use bindings::Windows::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};

use crate::{eventgrid::Signal, hardware::Key, replay::Command};

pub struct Recorder {
    recording: bool,
    records: Vec<Command>,
    out_file: PathBuf,
    rx: Receiver<Signal>,
}

impl Recorder {
    
    pub fn new(out_file: PathBuf, rx: Receiver<Signal>) -> Self {
        Self {
            recording: false,
            records: Vec::with_capacity(128),
            out_file,
            rx,
        }
    }

    pub fn start(&mut self) {
        info!("start recording or pause it later by pressing UP");
        while let Ok(signal) = self.rx.recv() {
            match signal {
                Signal::Input(key) if self.recording => self.record(key),
                Signal::Input(key) => warn!("discarding {:?} because recorder is paused", key),
                Signal::Pause => self.recording = !self.recording,
                Signal::Shutdown => break,
            }
        }
        info!("recording stopped");
        self.flush();
    }

    fn record(&mut self, key: Key) {
        if key.is_mouse() {
            let position= &mut POINT { x: 0, y: 0 };
            unsafe { GetCursorPos(position); }
            self.records.push(Command::MouseCommand{
                key: format!("{:?}", key),
                x: position.x,
                y: position.y,
            });
            info!("{:?} at ({}|{}) recorded", key, position.x, position.y);
        } else {
            self.records.push(Command::KeyboardCommand{ 
                key: format!("{:?}", key),
            });
            info!("{:?} recorded", key);
        }
    }

    fn flush(&self) {
        info!("writing {} records to {}", self.records.len(), self.out_file.to_string_lossy());
        match File::create(self.out_file.as_path())
            .map_err(|e| format!("failed to create file: {:?}", e.kind()))
            .and_then(|file| Ok(BufWriter::new(file)))
            .and_then(|writer| serde_yaml::to_writer(writer, &self.records).map_err(|e| e.to_string())) 
        {
            Ok(_) => info!("writing finished"),
            Err(what) => error!("{}", what),
        }            
    }
}
