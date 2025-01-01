use crate::capture::BoardCapturer;

pub struct ChessOverlayGUI {
    analyze_running: bool,
    capturer: BoardCapturer,
    texture: egui::TextureHandle,
}

impl ChessOverlayGUI {
    pub fn new(context: &eframe::CreationContext) -> Self  {
        Self {
            analyze_running: false,
            capturer: Default::default(),
            texture: context.egui_ctx.load_texture(
                "noise",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
        }
    }
}

impl eframe::App for ChessOverlayGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|  {
            if self.analyze_running {
                if ui.button("Stop analysis").clicked() {
                    self.analyze_running = false;
                    return;
                }
                self.capturer.run_capture();
                let (image, width, height) = self.capturer.last_rgba_image();
                self.texture.set(
                    egui::ColorImage {
                        size: [width, height],
                        pixels: image
                    },
                    egui::TextureOptions::NEAREST,
                );
                let size = self.texture.size_vec2();
                let sized_texture = egui::load::SizedTexture::new(&self.texture, size);
                ui.add(egui::Image::new(sized_texture).shrink_to_fit());
            } else {
                if ui.button("Start analysis").clicked() {
                    self.analyze_running = true;
                }
            }
        });
    }
}