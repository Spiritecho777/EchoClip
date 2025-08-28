use once_cell::sync::OnceCell;
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItemBuilder, MenuEvent}, Icon, TrayIcon};
use std::sync::mpsc::{Sender, channel};

pub fn init_tray(tx: Sender<()>) -> TrayIcon {
    // On "leak" les MenuItem pour qu'ils vivent 'static
    let show_item = Box::leak(Box::new(MenuItemBuilder::new().text("Afficher").build()));
    let quit_item = Box::leak(Box::new(MenuItemBuilder::new().text("Quitter").build()));

    let show_id = show_item.id();
    let quit_id = quit_item.id();

    let menu = Menu::new();
    menu.append(show_item);
    menu.append(quit_item);

    //let icon_data = vec![0u8; 16 * 16 * 4];
    //let icon = Icon::from_rgba(icon_data, 16, 16).unwrap();
    let mut icon_data = Vec::new();
    for _ in 0..(16 * 16) {
        icon_data.extend_from_slice(&[255, 0, 0, 255]); // rouge opaque
    }
    let icon = Icon::from_rgba(icon_data, 16, 16).unwrap();

    let tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Clipboard Manager")
        .with_menu(Box::new(menu))
        //.with_menu_on_left_click(true)
        .build()
        .unwrap();

    //TRAY.set(_tray).ok();

    let rx = MenuEvent::receiver();
    std::thread::spawn(move || {
        for event in rx {
            if event.id == show_id {
                println!("UI demo !");
                tx.send(()).unwrap();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });

    tray
}