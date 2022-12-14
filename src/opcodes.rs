use crate::cpu::AddressingMode;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub abbrv: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, abbrv: &'static str, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            abbrv,
            bytes,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPCODES: Vec<OpCode> = vec![
        // BRK
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        // INX
        OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
        // LDA
        OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xBD, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xB9, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::Indirect_X),
        OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Immediate),
        OpCode::new(0xB1, "LDA", 2, 5 /* +1 if page crossed */, AddressingMode::Indirect_Y),
        // STA
        OpCode::new(0x85, "STA", 2, 2, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 2, AddressingMode::ZeroPage_X),
        OpCode::new(0x8D, "STA", 3, 3, AddressingMode::Absolute),
        OpCode::new(0x9D, "STA", 3, 3, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 3, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 2, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 2, AddressingMode::Indirect_Y),
        // TAX
        OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpu_op in CPU_OPCODES.iter() {
            map.insert(cpu_op.code, cpu_op);
        }
        map
    };
}
