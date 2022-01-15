pub fn micro_code_view(
    ui: &mut eframe::egui::Ui,
    micro_code_addr: usize,
    micro_code: &mut crate::vm::MicroCode,
) {
    ui.horizontal(|ui| {
        ui.label(format!("{:04X}H", micro_code_addr));
        combo_box_registers_and_switch(ui, micro_code_addr * 10, &mut micro_code.x_bus);
        combo_box_registers_and_switch(ui, micro_code_addr * 10 + 1, &mut micro_code.y_bus);
        combo_box_alu(ui, micro_code_addr * 10 + 2, &mut micro_code.alu);
        combo_box_sft(ui, micro_code_addr * 10 + 3, &mut micro_code.sft);
        combo_box_bool(ui, micro_code_addr * 10 + 4, &mut micro_code.sin);
        combo_box_bool(ui, micro_code_addr * 10 + 5, &mut micro_code.fl);
        combo_box_registers(ui, micro_code_addr * 10 + 6, &mut micro_code.z_bus);
        combo_box_mem(ui, micro_code_addr * 10 + 7, &mut micro_code.mem);
        combo_box_branch(ui, micro_code_addr * 10 + 8, &mut micro_code.branch);
        combo_box_bool(ui, micro_code_addr * 10 + 9, &mut micro_code.hlt);
        ui.add(eframe::egui::widgets::DragValue::new(&mut micro_code.addr));
    });
}

use crate::vm::{AluOp, Branch, MemOp, Register, RegisterOrSwitch, ShiftOp};

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
fn combo_box_registers_and_switch(
    ui: &mut eframe::egui::Ui,
    id: usize,
    register: &mut crate::vm::RegisterOrSwitch,
) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(register.to_string())
        .show_ui(ui, |ui| {
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
        });
}
fn combo_box_registers(ui: &mut eframe::egui::Ui, id: usize, register: &mut crate::vm::Register) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(register.to_string())
        .show_ui(ui, |ui| {
            for selectable in REGISTER_SELECTABLE {
                ui.selectable_value(register, selectable, selectable.to_string());
            }
        });
}
const ALU_OP_SELECTABLE: [AluOp; 7] = [
    AluOp::XPlusY,
    AluOp::XMinusY,
    AluOp::XAndY,
    AluOp::XorY,
    AluOp::XxorY,
    AluOp::XPlus1,
    AluOp::XMinus1,
];
fn combo_box_alu(ui: &mut eframe::egui::Ui, id: usize, alu_op: &mut AluOp) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(alu_op.to_string())
        .show_ui(ui, |ui| {
            for selectable in ALU_OP_SELECTABLE {
                ui.selectable_value(alu_op, selectable, selectable.to_string());
            }
        });
}
const SHIFT_OP_SELECTABLE: [ShiftOp; 7] = [
    ShiftOp::Nop,
    ShiftOp::RRwC,
    ShiftOp::RlwC,
    ShiftOp::SRL,
    ShiftOp::SLL,
    ShiftOp::SRA,
    ShiftOp::SLA,
];
fn combo_box_sft(ui: &mut eframe::egui::Ui, id: usize, shift_op: &mut ShiftOp) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(shift_op.to_string())
        .show_ui(ui, |ui| {
            for selectable in SHIFT_OP_SELECTABLE {
                ui.selectable_value(shift_op, selectable, selectable.to_string());
            }
        });
}
fn combo_box_bool(ui: &mut eframe::egui::Ui, id: usize, register: &mut bool) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(if *register { "1" } else { "0" })
        .show_ui(ui, |ui| {
            ui.selectable_value(register, false, "0");
            ui.selectable_value(register, true, "1");
        });
}
const MEM_OP_SELECTABLE: [MemOp; 3] = [MemOp::Nop, MemOp::R, MemOp::W];
fn combo_box_mem(ui: &mut eframe::egui::Ui, id: usize, mem_op: &mut MemOp) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(mem_op.to_string())
        .show_ui(ui, |ui| {
            for selectable in MEM_OP_SELECTABLE {
                ui.selectable_value(mem_op, selectable, selectable.to_string());
            }
        });
}
const BRANCH_OP_SELECTABLE: [Branch; 7] = [
    Branch::Plus1,
    Branch::J,
    Branch::JM,
    Branch::JZ,
    Branch::JC,
    Branch::JV,
    Branch::JI,
];
fn combo_box_branch(ui: &mut eframe::egui::Ui, id: usize, branch: &mut Branch) {
    eframe::egui::ComboBox::from_id_source(id)
        .selected_text(branch.to_string())
        .show_ui(ui, |ui| {
            for selectable in BRANCH_OP_SELECTABLE {
                ui.selectable_value(branch, selectable, selectable.to_string());
            }
        });
}
