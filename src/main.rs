struct Cpu {
    sp: u8,
    stack: [u16; 16],
    registers: RegisterBank,
    pc: u8,
    ram: [u8; 4096],
}

impl Cpu {
    fn default() -> Self {
        Cpu {
            sp: 0,
            registers: RegisterBank::default(),
            pc: 0,
            stack: [0; 16],
            ram: [0; 4096],
        }
    }
}

struct RegisterBank {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    VF: u8,
    I: u16,
}

impl RegisterBank {
    fn default() -> Self {
        Self {
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
            I: 0,
        }
    }
}

fn main() {
    // Reference:
    // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1

    let rom = include_bytes!("../roms/test_opcode.ch8");
    for byte in rom {
        print!("{:x} ", byte);
    }
    let cpu = Cpu::default();
}
