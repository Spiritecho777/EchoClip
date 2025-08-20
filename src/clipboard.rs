use arboard::Clipboard;
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use eframe::emath::History;

pub fn start_clipboard(history: Arc<Mutex<Vec<String>>>) {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        let mut last = String::new();
        
        loop{
            if let Ok(text) = clipboard.get_text(){
                if text != last {
                    last = text.clone();
                    let mut hist = history.lock().unwrap();
                    hist.push(text);
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
}