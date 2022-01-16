use crate::vm::{AluOp, Branch, MemOp, MicroArch, Register, RegisterOrSwitch, ShiftOp};
use eframe::egui::CtxRef;
use eframe::epi::Frame;

pub struct VMView {
    ///VM
    vm: crate::vm::MicroArch,
    open_register_view: bool,
    open_micro_code_view: bool,
    open_memory_view: bool,
    /// where the current viewing micro code page.
    current_viewing_page: u8,
    /// inter frame data tracking
    auto_exec: bool,
}
impl VMView {
    pub fn init() -> Self {
        Self {
            vm: MicroArch::construct(vec![
                crate::vm::MicroCode {
                    x_bus: RegisterOrSwitch::Register(Register::Nop),
                    y_bus: RegisterOrSwitch::Register(Register::Nop),
                    alu: AluOp::XPlusY,
                    sft: ShiftOp::Nop,
                    sin: false,
                    fl: false,
                    z_bus: Register::Nop,
                    mem: MemOp::Nop,
                    branch: Branch::Plus1,
                    hlt: false,
                    addr: 0
                };
                1 << 16
            ]),
            open_register_view: true,
            open_micro_code_view: true,
            open_memory_view: false,
            auto_exec: false,
            current_viewing_page: 0,
        }
    }
}
impl eframe::epi::App for VMView {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        let panel = eframe::egui::TopBottomPanel::top("windows");
        panel.show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Read CPU config & main memory").clicked() {
                        let cpu_and_memory: Option<crate::vm::MicroArch> = rfd::FileDialog::new()
                            .add_filter("マイクロコードとメインメモリ", &["cpu_memory"])
                            .pick_file()
                            .map(|path| {
                                std::fs::read(path).ok().map(|cpu_and_memory| {
                                    bincode::deserialize(&cpu_and_memory).ok()
                                })
                            })
                            .flatten()
                            .flatten();
                        if let Some(cpu_and_memory) = cpu_and_memory {
                            self.vm = cpu_and_memory;
                        }
                    }
                    if ui.button("Save CPU config & main memory").clicked() {
                        rfd::FileDialog::new()
                            .add_filter("マイクロコードとメインメモリ", &["cpu_memory"])
                            .save_file()
                            .map(|x| (x, bincode::serialize(&self.vm).ok()))
                            .map(|(path, vm)| {
                                if let Some(vm) = vm {
                                    std::fs::write(path, vm).ok();
                                }
                            });
                    }
                });
                ui.checkbox(&mut self.open_register_view, "Register View");
                ui.checkbox(&mut self.open_micro_code_view, "Microcode View");
                ui.checkbox(&mut self.open_memory_view, "Memory View");
            });
        });
        let register_view =
            eframe::egui::Window::new("RegisterView").open(&mut self.open_register_view);
        register_view.show(ctx, |ui| {
            crate::register_view::register_view(ui, &mut self.vm, &mut self.auto_exec)
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
                    ui.add(crate::hex_input::HexInput::new(
                        &mut self.current_viewing_page,
                        0xcafebabe,
                    ));
                    if ui.button(">").clicked() {
                        if self.current_viewing_page < 0xff {
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
                                if let Some(micro_code) =
                                    self.vm.micro_program.get_mut(micro_code_addr as usize)
                                {
                                    crate::micro_code_view::micro_code_view(
                                        ui,
                                        micro_code_addr as usize,
                                        micro_code,
                                    )
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
                crate::ram_view::ram_view(ui, &mut self.vm.memory);
            });
        if self.auto_exec {
            println!("Auto exec enabled");
            self.auto_exec = !self.vm.exec();
            _frame.request_repaint();
        }
    }
    fn on_exit(&mut self) {
        let path = rfd::FileDialog::new()
            .add_filter("マイクロコードとメインメモリ", &["cpu_memory"])
            .save_file();
        if let Some(path) = path {
            if let Ok(vm_persistence) = bincode::serialize(&self.vm) {
                std::fs::write(path, vm_persistence).ok();
            }
        } else {
            std::process::exit(0);
        }
    }
    fn name(&self) -> &str {
        "MicroProgramming"
    }
}
