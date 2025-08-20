mod clipboard;
mod systray;
mod ui;

use std::sync::{Arc, Mutex, mpsc};

fn main() {
    let history = Arc::new(Mutex::new(Vec::new()));
    clipboard::start_clipboard(history.clone());

    let (tx,rx) = mpsc::channel();
    systray::init_tray(tx);

    for _ in rx{
        ui::show_ui(history.clone());
    }
}