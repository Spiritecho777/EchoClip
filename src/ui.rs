use eframe::{egui, NativeOptions};
use std::sync::{Arc, Mutex};

pub fn show_ui(history: Arc<Mutex<Vec<String>>>){
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 400.0])
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Historique Presse-papiers",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp { history }) as Box<dyn eframe::App>)),
    ).unwrap();
}

struct MyApp {
    history: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Historique");
            let hist = self.history.lock().unwrap();
            for item in hist.iter().rev().take(20) {
                ui.label(item);
            }
        });
    }
}