use std::sync::atomic::{AtomicUsize, Ordering};

pub fn ram_view(ui: &mut eframe::egui::Ui, memory: &mut [u8]) {
    static COLUMNS: once_cell::sync::Lazy<AtomicUsize> =
        once_cell::sync::Lazy::new(|| AtomicUsize::new(2));
    let columns = COLUMNS.load(Ordering::Relaxed);
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button("<").clicked() & (columns > 1) {
                COLUMNS.store(columns - 1, Ordering::SeqCst);
            }
            ui.label(format!("{}", columns));
            if ui.button(">").clicked() & (columns < 0xff) {
                COLUMNS.store(columns + 1, Ordering::SeqCst);
            }
        });
        eframe::egui::containers::ScrollArea::both().show(ui, |ui| {
            ui.vertical(|ui| {
                if columns > 1 {
                    for (y, cells) in memory.chunks_mut(columns).enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:02X}", y * columns));
                            ui.columns(columns, |columns_ui| {
                                for (x, cell) in
                                    cells.iter_mut().zip(columns_ui.iter_mut()).enumerate()
                                {
                                    cell.1.add_sized(
                                        [20.0, 16.0],
                                        crate::hex_input::HexInput::new(cell.0, y * columns + x),
                                    );
                                }
                            });
                        });
                    }
                } else {
                    for (y, cell) in memory.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:02X}", y));
                            ui.add(crate::hex_input::HexInput::new(cell, y));
                        });
                    }
                }
            });
        });
    });
}
