use eframe::egui::Label;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn ram_view(ui: &mut eframe::egui::Ui, memory: &mut [u8]) {
    static COLUMNS: once_cell::sync::Lazy<AtomicUsize> =
        once_cell::sync::Lazy::new(|| AtomicUsize::new(2));
    let columns = COLUMNS.load(Ordering::Relaxed);
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("width");
            if ui.button("<").clicked() & (columns > 1) {
                COLUMNS.store(columns - 1, Ordering::SeqCst);
            }
            ui.label(format!("{}", columns));
            if ui.button(">").clicked() & (columns < 0xff) {
                COLUMNS.store(columns + 1, Ordering::SeqCst);
            }
        });
        eframe::egui::containers::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.columns(columns + 1, |columns_ui| {
                        for (y, cells) in memory.chunks_mut(columns).enumerate() {
                            columns_ui[0].add_sized(
                                [40.0, 18.0],
                                Label::new(format!("{:02X}", y * columns)),
                            );
                            for (x, cell) in cells
                                .iter_mut()
                                .zip(columns_ui.iter_mut().skip(1))
                                .enumerate()
                            {
                                cell.1
                                    .add(crate::hex_input::HexInput::new(cell.0, y * columns + x));
                            }
                        }
                    });
                });
            });
    });
}
