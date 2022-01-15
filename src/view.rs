use crate::vm::MicroArch;
use eframe::egui::CtxRef;
use eframe::epi::Frame;

pub struct VMView {
    ///VM
    vm: crate::vm::MicroArch,
    open_register_view:bool,
    /// where the current viewing micro code page.
    current_viewing_page: u8,
}
impl VMView {
    pub fn init() -> Self {
        Self {
            vm: MicroArch::construct(vec![]),
            open_register_view: true,
            current_viewing_page: 0,
        }
    }
}
impl eframe::epi::App for VMView {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        let panel = eframe::egui::TopBottomPanel::top("Windows");
        panel.show(ctx,|ui|{
            ui.checkbox(&mut self.open_register_view,"RegisterView");
        });
        let register_view = eframe::egui::Window::new("RegisterView").open(&mut self.open_register_view);
        register_view.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("R0~R6");
                        for (name, x) in self.vm.gpr.iter().enumerate() {
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
                            ui.label(format!("{:02X}H", self.vm.ir));
                            ui.label(format!("{:02X}H", self.vm.pc));
                            ui.label(format!("{:02X}H", self.vm.mar));
                            ui.label(format!("{:02X}H", self.vm.mdr));
                            ui.label(format!("{:02X}H", self.vm.str));
                        });
                        ui.vertical(|ui| {
                            ui.label("Minus flag");
                            ui.label("Zero flag");
                            ui.label("Carry flag");
                            ui.label("Overflow flag");
                        });
                        ui.vertical(|ui| {
                            ui.label(format!("{}", self.vm.str & 0x01));
                            ui.label(format!("{}", self.vm.str & 0x02 >> 1));
                            ui.label(format!("{}", self.vm.str & 0x04 >> 2));
                            ui.label(format!("{}", self.vm.str & 0x08 >> 3));
                        });
                    });
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("sw1");
                            ui.add(eframe::egui::widgets::DragValue::new(&mut self.vm.sw1));
                        });
                        ui.horizontal(|ui| {
                            ui.label("sw2");
                            ui.add(eframe::egui::widgets::DragValue::new(&mut self.vm.sw2));
                        });
                    })
                })
            })
        });
    }

    fn name(&self) -> &str {
        "MicroProgramming"
    }
}
