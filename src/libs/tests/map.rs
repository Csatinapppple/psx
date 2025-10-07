mod opcode_range {

    use crate::libs::map::opcode::Instruction;

    #[test]
    pub fn primary() {
        let opcode = 0b_111111_01011101110101101001011010;
        let result = Instruction(opcode);
        println!("{:032b}", result.0);
        assert_eq!(result.primary(), 0b111111);
    }

    #[test]
    pub fn rt() {
        let opcode = 0b10110101010_11111_0101101011101110;
        let result = Instruction(opcode);
        assert_eq!(result.rt(), 0b11111);
    }

    #[test]
    pub fn imm() {
        let opcode = 0b0101101010101010_1111111111111111_;
        let result = Instruction(opcode);
        assert_eq!(result.imm(), 0b11111111_11111111);
    }

    #[test]
    pub fn rs() {
        let opcode = 0b101100_11111_011010101011110101110;
        let result = Instruction(opcode);
        assert_eq!(result.rs(), 0b11111);
    }
}
