pub fn ram_view(ui: &mut eframe::egui::Ui, memory: &mut [u8]) {
    let x = memory.chunks_mut(16).enumerate();
    ui.vertical(|ui| {
        for (y, cells) in x {
            ui.horizontal(|ui| {
                ui.label(format!("{:02X}", y << 4));
                for (x, cell) in cells.iter_mut().enumerate() {
                    ui.add(crate::hex_input::HexInput::new(cell, y * 16 + x));
                }
            });
        }
    });
}
