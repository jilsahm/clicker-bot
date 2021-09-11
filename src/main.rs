use std::sync::mpsc::channel;
use clicker_bot::keyboard::KeyboardMonitor;
use clicker_bot::mouse::VirtualMouse;

fn main() {
    env_logger::builder().parse_filters("info").init();
    let (tx, rx) = channel();
    KeyboardMonitor::new(tx).start();
    let mouse = VirtualMouse::new(rx);
    mouse.start();
}