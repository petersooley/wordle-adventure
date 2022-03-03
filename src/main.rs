use eframe::{egui, epi};

struct Wordle;

impl epi::App for Wordle {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                let mut frame = egui::Frame::group(ui.style());
                frame.stroke = egui::Stroke::new(2.0, egui::Color32::GRAY);
                frame.show(ui, |ui| {
                    ui.label("h");
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Wordle"
    }
}

fn main() {
    let app = Wordle;
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), options);
}
