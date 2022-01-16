pub fn micro_code_view(
    ui: &mut eframe::egui::Ui,
    micro_code_base_addr: usize,
    micro_code_addr: usize,
    micro_codes: &mut [crate::vm::MicroCode],
) {
    const TABLE_NAME: [&str; 12] = [
        "Address",
        "X-Bus",
        "Y-Bus",
        "ALU",
        "SFT",
        "Sin",
        "FL",
        "Z-Bus",
        "Mem",
        "Branch",
        "Halt",
        "MicrocodeAddress",
    ];
    eframe::egui::ScrollArea::both().show(ui, |ui| {
        ui.columns(12, |columns| {
            for (x, name) in TABLE_NAME.iter().enumerate() {
                columns[x].label(*name);
            }
            for (x, micro_code) in micro_codes.iter_mut().enumerate() {
                columns[0].add_sized([64.0, 18.0], {
                    let addr = micro_code_base_addr + x;
                    Label::new(RichText::new(format!("{:04X}H", addr)).color(
                        if addr == micro_code_addr {
                            Color32::RED
                        } else {
                            Color32::WHITE
                        },
                    ))
                });
                combo_box_registers_and_switch(&mut columns[1], x * 10, &mut micro_code.x_bus);
                combo_box_registers_and_switch(&mut columns[2], x * 10 + 1, &mut micro_code.y_bus);
                combo_box_alu(&mut columns[3], x * 10 + 2, &mut micro_code.alu);
                combo_box_sft(&mut columns[4], x * 10 + 3, &mut micro_code.sft);
                combo_box_bool(&mut columns[5], x * 10 + 4, &mut micro_code.sin);
                combo_box_bool(&mut columns[6], x * 10 + 5, &mut micro_code.fl);
                combo_box_registers(&mut columns[7], x * 10 + 6, &mut micro_code.z_bus);
                combo_box_mem(&mut columns[8], x * 10 + 7, &mut micro_code.mem);
                combo_box_branch(&mut columns[9], x * 10 + 8, &mut micro_code.branch);
                combo_box_bool(&mut columns[10], x * 10 + 9, &mut micro_code.hlt);
                columns[11].add(eframe::egui::widgets::DragValue::new(&mut micro_code.addr));
            }
        });
    });
}

use crate::vm::{AluOp, Branch, MemOp, Register, RegisterOrSwitch, ShiftOp};
use eframe::egui::{Color32, Label, RichText};

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
    ShiftOp::Srl,
    ShiftOp::Sll,
    ShiftOp::Sra,
    ShiftOp::Sla,
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
