mod clipboard;
mod systray;
mod ui;

use std::sync::{Arc, Mutex, mpsc};

fn main() {
    // Historique du presse-papiers
    let history = Arc::new(Mutex::new(Vec::new()));
    clipboard::start_clipboard(history.clone());

    // Channel pour déclencher l'UI depuis le systray
    let (tx, rx) = mpsc::channel();

    // Initialise le systray et garde l'icône en vie
    let _tray = systray::init_tray(history.clone(),tx.clone());

    // Boucle principale pour afficher l'UI
    /*for _ in rx {
        println!("UI demandée !");
        ui::show_ui(history.clone());
    }*/
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}