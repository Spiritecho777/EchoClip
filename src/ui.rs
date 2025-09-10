use eframe::{egui, NativeOptions};
use std::{thread, time::{Duration}, sync::{Arc, Mutex,atomic::{AtomicBool, Ordering},}};
use crate::UI_VISIBLE;

pub fn show_ui(history: Arc<Mutex<Vec<String>>>,show_flag: Arc<AtomicBool>) {
    thread::spawn(move || {
        println!("UI thread started");
        loop {
            if show_flag.swap(false, Ordering::Relaxed) {
                println!("Launching UI...");
                let options = NativeOptions {
                    viewport: egui::ViewportBuilder::default()
                        .with_inner_size([300.0, 400.0])
                        .with_decorations(true)
                        .with_always_on_top()
                        .with_transparent(false),
                    ..Default::default()
                };

                let mut app = MyApp {
                    history: history.clone(),
                    visible: true,
                    show_flag: show_flag.clone(),
                };

                eframe::run_native(
                    "Historique Presse-papiers",
                    options,
                    Box::new(|_cc| Ok(Box::new(app) as Box<dyn eframe::App>)),
                ).ok();

                println!("UI closed with result:");
            }
            // Ajoute un blocage ici pour éviter que le thread se termine
            thread::sleep(Duration::from_secs(100));
        }
    });

    struct MyApp {
        history: Arc<Mutex<Vec<String>>>,
        visible: bool,
        show_flag: Arc<AtomicBool>,
    }

    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            if self.show_flag.swap(false, Ordering::SeqCst) {
                self.visible = !self.visible;
                UI_VISIBLE.store(self.visible, Ordering::SeqCst);
            }

            if self.visible {
                ctx.request_repaint();
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Historique");
                    let hist = self.history.lock().unwrap();
                    for item in hist.iter().rev().take(20) {
                        ui.label(item);
                    }
                });
            }
        }
    }
}