pub struct Instruction(pub u32);

/*
    Little-Endian bit table
    31-30-29-...-0
*/

impl Instruction {
    // return primary bits of an opcode [31:26]
    pub fn primary(&self) -> u32 {
        let Instruction(op) = self;
        op >> 26
    }

    // return register index in bits [20:16] rt
    pub fn rt(&self) -> u32 {
        let Instruction(op) = self;
        (op >> 16) & 0b1_1111
    }

    // return register index in bits [15:0] immediate16bit
    pub fn imm(&self) -> u32 {
        let Instruction(op) = self;
        op & 0xffff
    }

    // return register index in bits [25:21] rs
    pub fn rs(&self) -> u32 {
        let Instruction(op) = self;
        (op >> 21) & 0b1_1111
    }
}
