use eframe::egui::mutex::RwLock;
use eframe::egui::{Response, Ui};
use std::sync::Arc;

/// Hexadecimal input.
///
///
pub struct HexInput<'a> {
    tgt: &'a mut u8,
    buffer: Arc<RwLock<String>>,
    key: usize,
}
impl<'a> HexInput<'a> {
    pub fn new(target: &'a mut u8, buffer: Arc<RwLock<String>>, key: usize) -> Self {
        Self {
            tgt: target,
            buffer,
            key,
        }
    }
}

static KEY: once_cell::sync::Lazy<RwLock<usize>> = once_cell::sync::Lazy::new(|| RwLock::new(0));
impl<'a> eframe::egui::Widget for HexInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text_buffer = if self.key == *KEY.read() {
            let buffer = self.buffer.read().clone();
            if buffer != self.tgt.to_string() {
                format!("{:02X}", self.tgt)
            } else {
                buffer
            }
        } else {
            format!("{:02X}", self.tgt)
        };

        let text_edit = eframe::egui::TextEdit::singleline(&mut text_buffer).desired_width(16.0);

        let response = ui.add(text_edit);

        if response.has_focus() {
            if let Ok(v) = u8::from_str_radix(&text_buffer, 16) {
                *self.tgt = v;
                *self.buffer.write() = text_buffer;
                *KEY.write() = self.key;
            }
        }
        response
    }
}
