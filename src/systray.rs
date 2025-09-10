use tray_icon::{TrayIconBuilder, Icon, menu::{Menu, MenuItem, MenuEvent}, };
use winit::{event_loop::{EventLoop, ControlFlow}, platform::windows::EventLoopBuilderExtWindows };
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use winit::event::Event;
use winit::event_loop::ActiveEventLoop;

pub fn init_tray(history: Arc<Mutex<Vec<String>>>, show_flag: Arc<AtomicBool>) {
    // Menu
    let mut menu = Menu::new();
    let show = MenuItem::new("Afficher", true, None);
    let quit = MenuItem::new("Quitter", true, None);

    menu.append(&show).unwrap();
    menu.append(&quit).unwrap();

    let icon_data = vec![255u8, 0, 0, 255].repeat(16*16);
    let icon = Icon::from_rgba(icon_data, 16, 16).unwrap();

    // EventLoop Winit
    let event_loop = EventLoop::<String>::with_user_event()
        .with_any_thread(true)
        .build()
        .unwrap();
    let proxy = event_loop.create_proxy();

    let show_id = show.id().clone();
    let quit_id = quit.id().clone();

    // Gestion MenuEvent
    MenuEvent::set_event_handler(Some(move |event:MenuEvent| {
        if event.id == show_id {
            let _ = proxy.send_event("show".to_string());
        } else if event.id == quit_id {
            let _ = proxy.send_event("quit".to_string());
        }
    }));

    // Crée TrayIcon
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("Historique Presse-papiers")
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()
        .unwrap();

    // Thread Winit pour message loop
    event_loop.run(move |event , target:&ActiveEventLoop | {
        target.set_control_flow(ControlFlow::Wait);

        if let winit::event::Event::UserEvent(msg) = event {
            match msg.as_str() {
                "show" => {
                    println!("Show event received");
                    show_flag.store(true,Ordering::SeqCst);
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });
}


