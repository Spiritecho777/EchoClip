use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItemBuilder, MenuEvent}, Icon};
use std::sync::mpsc::Sender;

/*pub fn init_tray(tx: Sender<()>){
    //let icon = Icon::from_file("icon.png").unwrap();

    let show_item = MenuItemBuilder::new().text("Afficher").build();
    let quit_item = MenuItemBuilder::new().text("Quitter").build();

    let show_id = show_item.id();
    let quit_id = quit_item.id();

    let menu = Menu::new();
    menu.append(&show_item);
    menu.append(&quit_item);

    let icon_data = vec![0u8; 16 * 16 * 4]; // RGBA noir transparent
    let icon = Icon::from_rgba(icon_data, 16, 16).expect("Échec de création de l'icône");

    let _tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Clipboard Manager")
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    let rx = MenuEvent::receiver();
    std::thread::spawn(move || {
        for event in rx {
            if event.id == show_id {
                tx.send(()).unwrap();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });
}*/

pub fn init_tray(tx: Sender<()>) {
    // On "leak" les MenuItem pour qu'ils vivent 'static
    let show_item = Box::leak(Box::new(MenuItemBuilder::new().text("Afficher").build()));
    let quit_item = Box::leak(Box::new(MenuItemBuilder::new().text("Quitter").build()));

    let show_id = show_item.id();
    let quit_id = quit_item.id();

    let menu = Menu::new();
    menu.append(show_item);
    menu.append(quit_item);

    let icon_data = vec![0u8; 16 * 16 * 4];
    let icon = Icon::from_rgba(icon_data, 16, 16).unwrap();

    let _tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Clipboard Manager")
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    let rx = MenuEvent::receiver();
    std::thread::spawn(move || {
        for event in rx {
            if event.id == show_id {
                tx.send(()).unwrap();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });
}

