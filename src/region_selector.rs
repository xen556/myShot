use eframe::{egui, App};
use egui::{Pos2, Rect};
use std::sync::{Arc, Mutex};

pub fn background() -> Option<(u32, u32, u32, u32)> {
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    
    eframe::run_native(
        "Overlay",
        options,
        Box::new(|_| {
            Ok(Box::new(RegionApp {
                start: None,
                end: None,
                result: result_clone,
            }))
        }),
    ).ok();
    
    result.lock().unwrap().clone()
}

struct RegionApp {
    start: Option<Pos2>,
    end: Option<Pos2>,
    result: Arc<Mutex<Option<(u32, u32, u32, u32)>>>,
}

impl App for RegionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgba_premultiplied(0,0,0,1))
            )
            .show(ctx, |ui| {
                let response = ui.allocate_response(
                    ui.available_size(),
                    egui::Sense::click_and_drag(),
                );
                
                if response.drag_started() {
                    self.start = response.interact_pointer_pos();
                }
                
                if response.dragged() {
                    self.end = response.interact_pointer_pos();
                }
                
                if response.drag_stopped() {
                    if let (Some(start), Some(end)) = (self.start, self.end) {
                        let rect = Rect::from_two_pos(start, end);
                        *self.result.lock().unwrap() = Some((
                            rect.min.x as u32,
                            rect.min.y as u32,
                            rect.width() as u32,
                            rect.height() as u32,
                        ));
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                
                if let (Some(start), Some(end)) = (self.start, self.end) {
                    let rect = Rect::from_two_pos(start, end);
                    
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_rgba_premultiplied(100,100,100,1),
                    );
                    
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                        egui::epaint::StrokeKind::Outside,
                    );
                }
            });
    }
}