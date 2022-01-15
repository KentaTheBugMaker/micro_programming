use serde::Deserialize;
use serde::Serialize;

/// Small 8 bit micro code driven architecture.
///
///
#[derive(Deserialize, Serialize)]
pub struct MicroArch {
    /// next micro code execution address
    pub micro_program_counter: u16,
    ///
    pub micro_program: Vec<MicroCode>,
    pub memory: Vec<u8>,
    pub gpr: [u8; 7],
    pub pc: u8,
    pub ir: u8,
    pub mdr: u8,
    pub mar: u8,
    pub str: u8,
    pub sw1: u8,
    pub sw2: u8,
    pub hlt: bool,
}
impl MicroArch {
    /// initialize registers and load micro code.
    pub fn construct(micro_codes: Vec<MicroCode>) -> Self {
        Self {
            micro_program_counter: 0,
            micro_program: micro_codes,
            memory: vec![],
            gpr: [0; 7],
            pc: 0,
            ir: 0,
            mdr: 0,
            mar: 0,
            str: 0,
            sw1: 0,
            sw2: 0,
            hlt: false,
        }
    }
    pub fn exec_until_hlt(&mut self) {
        while !self.hlt {
            self.exec()
        }
    }
    /// execute 1 microcode .
    pub fn exec(&mut self) {
        if !self.hlt {
            // fetch micro code .
            let micro_code = self.micro_program[self.micro_program_counter as usize];

            let x_bus = self.data_load(micro_code.x_bus);
            let y_bus = self.data_load(micro_code.y_bus);
            let alu_out = match micro_code.alu {
                AluOp::XPlusY => x_bus.wrapping_add(y_bus),
                AluOp::XMinusY => x_bus.wrapping_sub(y_bus),
                AluOp::XAndY => x_bus & y_bus,
                AluOp::XorY => x_bus | y_bus,
                AluOp::XxorY => x_bus ^ y_bus,
                AluOp::XPlus1 => x_bus + 1,
                AluOp::XMinus1 => x_bus - 1,
            };

            if micro_code.fl {
                if (alu_out & MSB) == MSB {
                    self.str |= 0x01;
                } else {
                    self.str &= 0b11111110;
                }
                if alu_out == 0 {
                    self.str |= 0x02;
                } else {
                    self.str &= 0b11111101;
                }
                if (x_bus as usize) + (y_bus as usize) > 255 {
                    self.str |= 0x04;
                } else {
                    self.str &= 0b11111011;
                }
                let test = (x_bus as isize) + (y_bus as isize);
                if (127 < test) | (test < -128) {
                    self.str |= 0x08;
                } else {
                    self.str &= 0b11110111;
                }
            }
            //extract carry flag
            let cf = (self.str & 0b00000100) >> 2;

            const MSB: u8 = 0b10000000;
            const LSB: u8 = 0b00000001;

            let z_bus = match micro_code.sft {
                ShiftOp::Nop => alu_out,
                ShiftOp::RRwC => {
                    //
                    self.str |= (alu_out & LSB) << 2;
                    let x = alu_out >> 1;
                    if cf == 1 {
                        x | MSB
                    } else {
                        x
                    }
                }
                ShiftOp::RlwC => {
                    self.str |= (alu_out & MSB) >> 4;
                    let x = alu_out << 1;
                    if cf == 1 {
                        x | LSB
                    } else {
                        x
                    }
                }
                ShiftOp::SRL => {
                    self.str |= (alu_out & LSB) << 2;
                    let x = alu_out >> 1;
                    if micro_code.sin {
                        x | MSB
                    } else {
                        x
                    }
                }
                ShiftOp::SLL | ShiftOp::SLA => {
                    self.str |= (alu_out & MSB) >> 4;
                    let x = alu_out << 1;
                    if micro_code.sin {
                        x | LSB
                    } else {
                        x
                    }
                }
                ShiftOp::SRA => {
                    let msb = alu_out & MSB;
                    self.str |= (alu_out >> 1) << 2;
                    msb | (alu_out >> 1)
                }
            };
            match micro_code.z_bus {
                Register::Nop => {}
                Register::R0 => self.gpr[0] = z_bus,
                Register::R1 => self.gpr[1] = z_bus,
                Register::R2 => self.gpr[2] = z_bus,
                Register::R3 => self.gpr[3] = z_bus,
                Register::R4 => self.gpr[4] = z_bus,
                Register::R5 => self.gpr[5] = z_bus,
                Register::R6 => self.gpr[6] = z_bus,
                Register::Pc => self.pc = z_bus,
                Register::Ir => self.ir = z_bus,
                Register::Mdr => self.mdr = z_bus,
                Register::Mar => self.mar = z_bus,
                Register::Str => self.str = z_bus,
            }
            match micro_code.mem {
                MemOp::Nop => {}
                MemOp::R => self.mdr = self.memory[self.mar as usize],
                MemOp::W => self.memory[self.mar as usize] = self.mdr,
            }
            match micro_code.branch {
                Branch::Plus1 => self.micro_program_counter += 1,
                Branch::J => self.micro_program_counter = micro_code.addr,
                Branch::JM => {
                    if (self.str & 0x01) == 0x01 {
                        self.micro_program_counter = micro_code.addr;
                    } else {
                        self.micro_program_counter += 1;
                    }
                }
                Branch::JZ => {
                    if (self.str & 0x02) == 0x02 {
                        self.micro_program_counter = micro_code.addr;
                    } else {
                        self.micro_program_counter += 1;
                    }
                }
                Branch::JC => {
                    if (self.str & 0x04) == 0x04 {
                        self.micro_program_counter = micro_code.addr;
                    } else {
                        self.micro_program_counter += 1;
                    }
                }
                Branch::JV => {
                    if (self.str & 0x08) == 0x08 {
                        self.micro_program_counter = micro_code.addr;
                    } else {
                        self.micro_program_counter += 1;
                    }
                }
                Branch::JI => self.micro_program_counter += self.ir as u16,
            }

            self.hlt = micro_code.hlt;
        }
    }
    fn data_load(&self, from: RegisterOrSwitch) -> u8 {
        match from {
            RegisterOrSwitch::Sw1 => self.sw1,
            RegisterOrSwitch::Sw2 => self.sw2,
            RegisterOrSwitch::Register(register) => match register {
                Register::Nop => 0,
                Register::R0 => self.gpr[0],
                Register::R1 => self.gpr[1],
                Register::R2 => self.gpr[2],
                Register::R3 => self.gpr[3],
                Register::R4 => self.gpr[4],
                Register::R5 => self.gpr[5],
                Register::R6 => self.gpr[6],
                Register::Pc => self.pc,
                Register::Ir => self.ir,
                Register::Mdr => self.mdr,
                Register::Mar => self.mar,
                Register::Str => self.str,
            },
        }
    }
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct MicroCode {
    pub x_bus: RegisterOrSwitch,
    pub y_bus: RegisterOrSwitch,
    pub alu: AluOp,
    pub sft: ShiftOp,
    pub sin: bool,
    pub fl: bool,
    pub z_bus: Register,
    pub mem: MemOp,
    pub branch: Branch,
    pub hlt: bool,
    pub addr: u16,
}
///This architecture use 42bit micro code
impl Assemble for MicroCode {
    fn assemble(&self) -> u64 {
        // high                                  low
        //  4 4  3   3   1  1  4  2     3    1   16
        //  x|y|alu|sft|sin|fl|z|mem|branch|hlt|addr
        self.addr.assemble()
            | self.hlt.assemble() << 16
            | self.branch.assemble() << 17
            | self.mem.assemble() << 20
            | self.z_bus.assemble() << 22
            | self.fl.assemble() << 26
            | self.sin.assemble() << 27
            | self.sft.assemble() << 28
            | self.alu.assemble() << 31
            | self.y_bus.assemble() << 33
            | self.x_bus.assemble() << 38
    }
}

trait Assemble {
    ///Assemble Microcode.
    fn assemble(&self) -> u64;
}
impl Assemble for u16 {
    fn assemble(&self) -> u64 {
        *self as u64
    }
}
impl Assemble for bool {
    fn assemble(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize,Eq, PartialEq)]
pub enum Branch {
    Plus1,
    J,
    JM,
    JZ,
    JC,
    JV,
    JI,
}
impl ToString for Branch{
    fn to_string(&self) -> String {
        match self{
            Branch::Plus1 => {"+1"}
            Branch::J => {"J"}
            Branch::JM => {"JM"}
            Branch::JZ => {"JZ"}
            Branch::JC => {"JC"}
            Branch::JV => {"JV"}
            Branch::JI => {"JI"}
        }.to_owned()
    }
}
impl Assemble for Branch {
    fn assemble(&self) -> u64 {
        match self {
            Branch::Plus1 => 0,
            Branch::J => 1,
            Branch::JM => 2,
            Branch::JZ => 3,
            Branch::JC => 4,
            Branch::JV => 5,
            Branch::JI => 6,
        }
    }
}
#[derive(Copy, Clone, Debug, Serialize, Deserialize,Eq, PartialEq)]
pub enum MemOp {
    Nop,
    R,
    W,
}
impl ToString for MemOp{
    fn to_string(&self) -> String {
        match self{
            MemOp::Nop => {""}
            MemOp::R => {"R"}
            MemOp::W => {"W"}
        }.to_owned()
    }
}
impl Assemble for MemOp {
    fn assemble(&self) -> u64 {
        match self {
            MemOp::Nop => 0,
            MemOp::R => 1,
            MemOp::W => 2,
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize,Eq, PartialEq)]
pub enum ShiftOp {
    Nop,
    RRwC,
    RlwC,
    SRL,
    SLL,
    SRA,
    SLA,
}
impl ToString for ShiftOp{
    fn to_string(&self) -> String {
        match self{
            ShiftOp::Nop => {""}
            ShiftOp::RRwC => {"RRwC"}
            ShiftOp::RlwC => {"RLwC"}
            ShiftOp::SRL => {"SRL"}
            ShiftOp::SLL => {"SLL"}
            ShiftOp::SRA => {"SRA"}
            ShiftOp::SLA => {"SLA"}
        }.to_owned()
    }
}
impl Assemble for ShiftOp {
    fn assemble(&self) -> u64 {
        match self {
            ShiftOp::Nop => 0,
            ShiftOp::RRwC => 1,
            ShiftOp::RlwC => 2,
            ShiftOp::SRL => 3,
            ShiftOp::SLL => 4,
            ShiftOp::SRA => 5,
            ShiftOp::SLA => 6,
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum RegisterOrSwitch {
    Sw1,
    Sw2,
    Register(Register),
}
impl ToString for RegisterOrSwitch {
    fn to_string(&self) -> String {
        match self {
            RegisterOrSwitch::Sw1 => "Sw1".to_owned(),
            RegisterOrSwitch::Sw2 => "Sw2".to_owned(),
            RegisterOrSwitch::Register(register) => register.to_string(),
        }
    }
}
impl Assemble for RegisterOrSwitch {
    fn assemble(&self) -> u64 {
        match self {
            RegisterOrSwitch::Sw1 => 13,
            RegisterOrSwitch::Sw2 => 14,
            RegisterOrSwitch::Register(register) => register.assemble(),
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Register {
    Nop,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    Pc,
    Ir,
    Mdr,
    Mar,
    Str,
}
impl ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::Nop => "",
            Register::R0 => "R0",
            Register::R1 => "R1",
            Register::R2 => "R2",
            Register::R3 => "R3",
            Register::R4 => "R4",
            Register::R5 => "R5",
            Register::R6 => "R6",
            Register::Pc => "PC",
            Register::Ir => "IR",
            Register::Mdr => "MDR",
            Register::Mar => "MAR",
            Register::Str => "STR",
        }
        .to_owned()
    }
}
impl Assemble for Register {
    fn assemble(&self) -> u64 {
        match self {
            Register::Nop => 0,
            Register::R0 => 1,
            Register::R1 => 2,
            Register::R2 => 3,
            Register::R3 => 4,
            Register::R4 => 5,
            Register::R5 => 6,
            Register::R6 => 7,
            Register::Pc => 8,
            Register::Ir => 9,
            Register::Mdr => 10,
            Register::Mar => 11,
            Register::Str => 12,
        }
    }
}
#[derive(Copy, Clone, Serialize, Deserialize,Eq, PartialEq)]
pub enum AluOp {
    XPlusY,
    XMinusY,
    XAndY,
    XorY,
    XxorY,
    XPlus1,
    XMinus1,
}
impl Assemble for AluOp {
    fn assemble(&self) -> u64 {
        match self {
            AluOp::XPlusY => 0,
            AluOp::XMinusY => 1,
            AluOp::XAndY => 2,
            AluOp::XorY => 3,
            AluOp::XxorY => 4,
            AluOp::XPlus1 => 5,
            AluOp::XMinus1 => 6,
        }
    }
}
impl ToString for AluOp {
    fn to_string(&self) ->String {
        match self {
            AluOp::XPlusY => "X+Y",
            AluOp::XMinusY => "X-Y",
            AluOp::XAndY => "X&Y",
            AluOp::XorY => "X|Y",
            AluOp::XxorY => "X^Y",
            AluOp::XPlus1 => "X+1",
            AluOp::XMinus1 => "X-1",
        }.to_owned()
    }
}
