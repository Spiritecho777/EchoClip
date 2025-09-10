mod clipboard;
mod systray;
mod ui;

use std::{thread,time::{Duration}, sync::{Arc, Mutex,atomic::{AtomicBool, }}};

static UI_VISIBLE:AtomicBool = AtomicBool::new(false);

fn main() {
    // Historique du presse-papiers
    let history = Arc::new(Mutex::new(Vec::new()));
    clipboard::start_clipboard(history.clone());

    let history_clone = history.clone();
    let show_flag = Arc::new(AtomicBool::new(false));


        let history_clone = history_clone.clone();
        let show_flag_clone = show_flag.clone();
    thread::spawn({move || {
            systray::init_tray(history_clone, show_flag_clone);
        }
    });

    println!("Launching UI thread...");
    ui::show_ui(history.clone(), show_flag.clone());

    /*let show_flag_clone = show_flag.clone();
    let history_clone = history.clone();
    thread::spawn(move || {
        ui::show_ui(history_clone,show_flag_clone)
    });*/

    loop{ thread::sleep(Duration::from_millis(1)); }
}