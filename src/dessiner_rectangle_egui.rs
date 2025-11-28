//! pour d'autres exemples voir https://github.com/emilk/egui/tree/main/examples

use std::error::Error;

use eframe::NativeOptions;
use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::CornerRadius;
use eframe::egui::Pos2;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let native_options = NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Demo",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )?;
    Ok(())
}

struct App {
    iterations: u32,
    zoom: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            iterations: 0,
            zoom: 1f32,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Side panel with slider
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Controls");
            ui.add(egui::Slider::new(&mut self.iterations, 0..=10).text("Iterations"));
            ui.add(egui::Slider::new(&mut self.zoom, 1.0..=100.0).text("Zoom"));
        });

        // Central drawing panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // pour dessiner des formes arbitraires, on a besoin de Painter
            let (_response, painter) =
                ui.allocate_painter(egui::Vec2::new(800.0, 800.0), egui::Sense::drag());

            let rect = egui::Rect::from_min_size(Pos2::new(200.0, 200.0), egui::vec2(200.0, 150.0));
            painter.rect_filled(rect, CornerRadius::ZERO, Color32::RED);
        });
    }
}
