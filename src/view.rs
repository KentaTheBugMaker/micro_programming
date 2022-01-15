use crate::vm::MicroArch;
use eframe::egui::CtxRef;
use eframe::epi::Frame;

pub struct VMView {
    ///VM
    vm: crate::vm::MicroArch,
    open_register_view: bool,
    open_micro_code_view:bool,
    /// where the current viewing micro code page.
    current_viewing_page: u8,
}
impl VMView {
    pub fn init() -> Self {
        Self {
            vm: MicroArch::construct(vec![]),
            open_register_view: true,
            open_micro_code_view: true,
            current_viewing_page: 0,
        }
    }
}
impl eframe::epi::App for VMView {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        let panel = eframe::egui::TopBottomPanel::top("Windows");
        panel.show(ctx, |ui| {
            ui.checkbox(&mut self.open_register_view, "RegisterView");
        });
        let register_view =
            eframe::egui::Window::new("RegisterView").open(&mut self.open_register_view);
        register_view.show(ctx, |ui| {
            crate::register_view::register_view(ui, &mut self.vm)
        });
        
        let micro_code_base_addr = (self.current_viewing_page as u16)<<8;
        
        let micro_code_view =
            eframe::egui::Window::new("MicroCodeView").open(&mut self.open_micro_code_view);
        
        micro_code_view.show(ctx, |ui| {
        ui.vertical(|ui|{
            for (micro_code,id_source) in self.vm.micro_program[micro_code_base_addr as usize..((micro_code_base_addr+256)as usize)].iter_mut().zip((0..256).map(|x|{x*10})) {
                crate::micro_code_view::micro_code_view(ui,id_source,micro_code)
            } 
        });

        
        });
    }

    fn name(&self) -> &str {
        "MicroProgramming"
    }
}
