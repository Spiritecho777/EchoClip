/*use tray_icon::{TrayIconBuilder, Icon, TrayIcon,
                menu::{Menu, MenuItem, MenuEvent},
};
use std::sync::mpsc::Sender;

pub fn init_tray(tx: Sender<()>) -> TrayIcon {
    let mut menu = Menu::new();
    let show = MenuItem::new("Afficher", true, None);
    let quit =  MenuItem::new("Quitter", true, None);
    menu.append(&show).unwrap();
    menu.append(&quit).unwrap();

    let icon_data = vec![255u8,0,0,255].repeat(16*16);
    let icon = Icon::from_rgba(icon_data,16,16).unwrap();

    let tray= TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()
        .unwrap();

    let show_id = show.id().clone();
    let quit_id = quit.id().clone();

    std::thread::spawn(move || {
        for event in  MenuEvent::receiver(){
            if event.id == show_id {
                tx.send(()).unwrap();
            } else if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });

    tray
}*/

//mod ui;
use tray_icon::{
    TrayIconBuilder, Icon,
    menu::{Menu, MenuItem, MenuEvent},
    TrayIconEvent,
};
use winit::{
    event_loop::{EventLoop, ControlFlow},
};
use std::sync::{Arc, Mutex, mpsc::Sender};


pub fn init_tray(history: Arc<Mutex<Vec<String>>>, tx: Sender<()>) {
    // Menu
    let mut menu = Menu::new();
    let show = MenuItem::new("Afficher", true, None);
    let quit = MenuItem::new("Quitter", true, None);
    menu.append(&show).unwrap();
    menu.append(&quit).unwrap();

    let icon_data = vec![255u8, 0, 0, 255].repeat(16*16);
    let icon = Icon::from_rgba(icon_data, 16, 16).unwrap();

    // EventLoop Winit
    let event_loop = EventLoop::<()>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    // Gestion MenuEvent
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(());
    }));

    // Crée TrayIcon
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Historique Presse-papiers")
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()
        .unwrap();

    let show_id = show.id().clone();
    let quit_id = quit.id().clone();
    let history_clone = history.clone();
    // Thread Winit pour message loop
    //std::thread::spawn(move || {
        event_loop.run(move |event, target | {
            target.set_control_flow(ControlFlow::Wait);

            if let winit::event::Event::UserEvent(_) = event {
                // On déclenche l'UI quand le menu est cliqué
                //ui::show_ui(history_clone.clone());
                tx.send(()).unwrap();
            }
        });
    //});
}


