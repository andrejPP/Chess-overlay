use scrap::{Capturer, Display, Frame};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

pub struct BoardCapturer {
    capturer: Capturer,
    last_image: Vec<egui::Color32>
}

impl Default for BoardCapturer {
    fn default() -> Self {
        Self {
            capturer: {
                let display = Display::primary().expect("Couldn't find primary display.");
                Capturer::new(display).expect("Couldn't begin capture.")
            },
            last_image: Vec::new(),
        }
    }
}

impl BoardCapturer {
    pub fn run_capture(&mut self) {
    let (w, h) = (self.capturer.width(), self.capturer.height());

        loop {
            let buffer = match self.capturer.frame() {
                 Ok(buffer) => buffer,
                 Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(Duration::new(1, 0) / 60);
                        continue;
                    } else {
                        panic!("Error: {}", error);
                    }
                 }
            };
            self.last_image = argb_to_rgba_vec(&buffer, w, h);
            return;
        }
    }
    
    pub fn last_rgba_image(&self) -> (Vec<egui::Color32>, usize, usize) {
        (self.last_image.to_vec(), self.capturer.width(), self.capturer.height())
    }

}

fn argb_to_rgba_vec(frame: &Frame, width: usize, height: usize) -> Vec<egui::Color32> {
    let mut output = Vec::with_capacity(width * height);
    let stride = frame.len() / height;

    for y in 0..height {
        for x in 0..width {
            let i = stride * y + 4 * x;
            output.push(egui::Color32::from_rgba_premultiplied(
                frame[i], // Red
                frame[i + 1], // Green
                frame[i + 2], // Blue
                255, // Alpha
            ));
        }
    }
    output
}