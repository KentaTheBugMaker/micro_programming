use crate::vm::MicroArch;
use eframe::egui::CtxRef;
use eframe::epi::Frame;
use std::sync::Arc;

pub struct VMView {
    ///VM
    vm: crate::vm::MicroArch,
    open_register_view: bool,
    open_micro_code_view: bool,
    open_memory_view: bool,
    /// where the current viewing micro code page.
    current_viewing_page: u8,
    ///
    hex_edit_buffer: Arc<eframe::egui::mutex::RwLock<String>>,
}
impl VMView {
    pub fn init() -> Self {
        Self {
            vm: MicroArch::construct(vec![]),
            open_register_view: true,
            open_micro_code_view: true,
            open_memory_view: false,
            current_viewing_page: 0,
            hex_edit_buffer: Arc::new(Default::default()),
        }
    }
}
impl eframe::epi::App for VMView {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        let panel = eframe::egui::TopBottomPanel::bottom("windows");
        panel.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.open_register_view, "Register View");
                ui.checkbox(&mut self.open_micro_code_view, "Microcode View");
                ui.checkbox(&mut self.open_memory_view, "Memory View");
            });
        });
        let register_view =
            eframe::egui::Window::new("RegisterView").open(&mut self.open_register_view);
        register_view.show(ctx, |ui| {
            crate::register_view::register_view(ui, &mut self.vm)
        });

        let micro_code_base_addr = (self.current_viewing_page as u16) << 8;

        let micro_code_view =
            eframe::egui::Window::new("MicroCodeView").open(&mut self.open_micro_code_view);

        micro_code_view.show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("<").clicked() {
                        if self.current_viewing_page > 0 {
                            self.current_viewing_page -= 1;
                        }
                    }
                    ui.label(self.current_viewing_page.to_string());
                    if ui.button(">").clicked() {
                        if self.current_viewing_page < 15 {
                            self.current_viewing_page += 1;
                        }
                    }
                });
                eframe::egui::containers::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            for micro_code_addr in
                                micro_code_base_addr..(micro_code_base_addr + 256)
                            {
                                ui.label(format!("{:04X}H", micro_code_addr));
                            }
                        });
                        ui.vertical(|ui| {
                            for (micro_code_addr, id_source) in (micro_code_base_addr
                                ..(micro_code_base_addr + 256))
                                .map(|micro_code_addr| (micro_code_addr, micro_code_addr * 10))
                            {
                                println!("micro code address:{}", micro_code_addr);
                                if let Some(micro_code) =
                                    self.vm.micro_program.get_mut(micro_code_addr as usize)
                                {
                                    crate::micro_code_view::micro_code_view(
                                        ui,
                                        id_source as usize,
                                        micro_code,
                                    )
                                } else {
                                    ui.label("No micro code here");
                                }
                            }
                        });
                    })
                });
            });
        });
        eframe::egui::Window::new("Ram View")
            .open(&mut self.open_memory_view)
            .show(ctx, |ui| {
                crate::ram_view::ram_view(ui, self.hex_edit_buffer.clone(), &mut self.vm.memory);
            });
    }

    fn name(&self) -> &str {
        "MicroProgramming"
    }
}
