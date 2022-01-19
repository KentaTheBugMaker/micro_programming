pub fn micro_code_view(
    ui: &mut eframe::egui::Ui,
    micro_code_base_addr: usize,
    micro_code_addr: usize,
    micro_codes: &mut [crate::vm::MicroCode],
) {
    const TABLE_NAME: [&str; 12] = [
        "Address", "X-Bus", "Y-Bus", "ALU", "SFT", "Sin", "FL", "Z-Bus", "Mem", "Branch", "Halt",
        "B.Addr",
    ];
    eframe::egui::ScrollArea::both().show(ui, |ui| {
        ui.columns(12, |columns| {
            for (x, name) in TABLE_NAME.iter().enumerate() {
                columns[x].label(*name);
            }
            for (x, micro_code) in micro_codes.iter_mut().enumerate() {
                columns[0].add_sized([40.0, 18.0], {
                    let addr = micro_code_base_addr + x;
                    Label::new(
                        RichText::new(format!("{:04X}H", addr))
                            .color(if addr == micro_code_addr {
                                Color32::RED
                            } else {
                                Color32::WHITE
                            })
                            .monospace(),
                    )
                });
                columns[1].register_or_switch(&mut micro_code.x_bus, x * 10);
                columns[2].register_or_switch(&mut micro_code.y_bus, x * 10 + 1);
                columns[3].alu(&mut micro_code.alu, x * 10 + 2);
                columns[4].sft(&mut micro_code.sft, x * 10 + 3);
                columns[5].bool(&mut micro_code.sin, x * 10 + 4);
                columns[6].bool(&mut micro_code.fl, x * 10 + 5);
                columns[7].register(&mut micro_code.z_bus, x * 10 + 6);
                columns[8].mem(&mut micro_code.mem, x * 10 + 7);
                columns[9].branch(&mut micro_code.branch, x * 10 + 8);
                columns[10].bool(&mut micro_code.hlt, x * 10 + 9);
                columns[11].add(eframe::egui::widgets::DragValue::new(&mut micro_code.addr));
            }
        });
    });
}

use crate::vm::{AluOp, Branch, MemOp, Register, RegisterOrSwitch, ShiftOp};
use eframe::egui::{Color32, Label, Response, RichText, Ui};

const REGISTER_OR_SWITCH_SELECTABLE: [RegisterOrSwitch; 2] =
    [RegisterOrSwitch::Sw1, RegisterOrSwitch::Sw2];

const REGISTER_SELECTABLE: [Register; 13] = [
    Register::Nop,
    Register::R0,
    Register::R1,
    Register::R2,
    Register::R3,
    Register::R4,
    Register::R5,
    Register::R6,
    Register::Pc,
    Register::Ir,
    Register::Mdr,
    Register::Mar,
    Register::Str,
];

const ALU_OP_SELECTABLE: [AluOp; 7] = [
    AluOp::XPlusY,
    AluOp::XMinusY,
    AluOp::XAndY,
    AluOp::XorY,
    AluOp::XxorY,
    AluOp::XPlus1,
    AluOp::XMinus1,
];

const SHIFT_OP_SELECTABLE: [ShiftOp; 7] = [
    ShiftOp::Nop,
    ShiftOp::RRwC,
    ShiftOp::RlwC,
    ShiftOp::Srl,
    ShiftOp::Sll,
    ShiftOp::Sra,
    ShiftOp::Sla,
];

const MEM_OP_SELECTABLE: [MemOp; 3] = [MemOp::Nop, MemOp::R, MemOp::W];

const BRANCH_OP_SELECTABLE: [Branch; 7] = [
    Branch::Plus1,
    Branch::J,
    Branch::JM,
    Branch::JZ,
    Branch::JC,
    Branch::JV,
    Branch::JI,
];

// Ui に追加実装.
trait AdditionalWidget {
    fn register_or_switch(&mut self, register: &mut RegisterOrSwitch, id: usize) -> Response;
    fn register(&mut self, register: &mut Register, id: usize) -> Response;
    fn alu(&mut self, alu_op: &mut AluOp, id: usize) -> Response;
    fn sft(&mut self, sft_op: &mut ShiftOp, id: usize) -> Response;
    fn bool(&mut self, x: &mut bool, id: usize) -> Response;
    fn mem(&mut self, mem_op: &mut MemOp, id: usize) -> Response;
    fn branch(&mut self, branch_op: &mut Branch, id: usize) -> Response;
}
impl AdditionalWidget for Ui {
    fn register_or_switch(&mut self, register: &mut RegisterOrSwitch, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(register.to_string())
            .show_ui(self, |ui| {
                for selectable in REGISTER_OR_SWITCH_SELECTABLE {
                    ui.selectable_value(register, selectable, selectable.to_string());
                }
                for selectable in REGISTER_SELECTABLE {
                    ui.selectable_value(
                        register,
                        RegisterOrSwitch::Register(selectable),
                        selectable.to_string(),
                    );
                }
            })
            .response
    }

    fn register(&mut self, register: &mut Register, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(register.to_string())
            .show_ui(self, |ui| {
                for selectable in REGISTER_SELECTABLE {
                    ui.selectable_value(register, selectable, selectable.to_string());
                }
            })
            .response
    }

    fn alu(&mut self, alu_op: &mut AluOp, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(alu_op.to_string())
            .show_ui(self, |ui| {
                for selectable in ALU_OP_SELECTABLE {
                    ui.selectable_value(alu_op, selectable, selectable.to_string());
                }
            })
            .response
    }
    fn sft(&mut self, shift_op: &mut ShiftOp, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(shift_op.to_string())
            .show_ui(self, |ui| {
                for selectable in SHIFT_OP_SELECTABLE {
                    ui.selectable_value(shift_op, selectable, selectable.to_string());
                }
            })
            .response
    }

    fn bool(&mut self, register: &mut bool, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(if *register { "1" } else { "" })
            .show_ui(self, |ui| {
                ui.selectable_value(register, false, "");
                ui.selectable_value(register, true, "1");
            })
            .response
    }
    fn mem(&mut self, mem_op: &mut MemOp, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(mem_op.to_string())
            .show_ui(self, |ui| {
                for selectable in MEM_OP_SELECTABLE {
                    ui.selectable_value(mem_op, selectable, selectable.to_string());
                }
            })
            .response
    }

    fn branch(&mut self, branch: &mut Branch, id: usize) -> Response {
        eframe::egui::ComboBox::from_id_source(id)
            .selected_text(branch.to_string())
            .show_ui(self, |ui| {
                for selectable in BRANCH_OP_SELECTABLE {
                    ui.selectable_value(branch, selectable, selectable.to_string());
                }
            })
            .response
    }
}
