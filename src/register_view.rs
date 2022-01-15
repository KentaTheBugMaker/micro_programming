use eframe::egui::mutex::RwLock;
use std::sync::Arc;

pub fn register_view(
    ui: &mut eframe::egui::Ui,
    hex_edit_buffer: Arc<RwLock<String>>,
    vm: &mut crate::vm::MicroArch,
) {
    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("R0~R6");
                for (name, x) in vm.gpr.iter().enumerate() {
                    ui.label(format!("R{} {:02x}H", name, x));
                }
            });
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.label("IR");
                    ui.label("PC");
                    ui.label("MAR");
                    ui.label("MDR");
                    ui.label("STR");
                });
                ui.vertical(|ui| {
                    ui.label(format!("{:02X}H", vm.ir));
                    ui.label(format!("{:02X}H", vm.pc));
                    ui.label(format!("{:02X}H", vm.mar));
                    ui.label(format!("{:02X}H", vm.mdr));
                    ui.label(format!("{:02X}H", vm.str));
                });
                ui.vertical(|ui| {
                    ui.label("Minus flag");
                    ui.label("Zero flag");
                    ui.label("Carry flag");
                    ui.label("Overflow flag");
                });
                ui.vertical(|ui| {
                    ui.label(format!("{}", vm.str & 0x01));
                    ui.label(format!("{}", vm.str & 0x02 >> 1));
                    ui.label(format!("{}", vm.str & 0x04 >> 2));
                    ui.label(format!("{}", vm.str & 0x08 >> 3));
                });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("sw1");
                    ui.add(crate::hex_input::HexInput::new(
                        &mut vm.sw1,
                        hex_edit_buffer.clone(),
                        0xfffff,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label("sw2");
                    ui.add(crate::hex_input::HexInput::new(
                        &mut vm.sw2,
                        hex_edit_buffer,
                        0xffffff,
                    ));
                });
            })
        })
    });
}
