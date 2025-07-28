pub struct Instruction(u32);

impl Instruction {
    // return primary bits of an opcode [31:26]
    pub fn primary(&self) -> u32 {
        let Instruction(op) = self;
        op >> 26
    }

    // return register index in bits [20:16]
    pub fn t(&self) -> u32 {
        let Instruction(op) = self;
        (op >> 16) & 0x1f
    }

    // return register index in bits [16:0]
    pub fn imm(&self) -> u32 {
        let Instruction(op) = self;
        op & 0xffff
    }

    // return source register in bits [21:25]
    pub fn s(&self) -> u32 {
        let Instruction(op) = self;
        (op >> 21) & 0b1_1111
    }

    pub fn get(&self) -> u32 {
        self
    }
}
