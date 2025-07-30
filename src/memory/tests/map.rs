mod opcode_range {

    use crate::memory::map::opcode;

    #[test]
    pub fn primary() {
        let opcode = 0b_111111_01011101110101101001011010;
        let result = opcode::PRIMARY.get(opcode);
        println!("{:032b}", result);
        assert_eq!(result, 0b111111);
    }

    #[test]
    pub fn rt() {
        let opcode = 0b10110101010_11111_0101101011101110;
        let result = opcode::RT.get(opcode);
        assert_eq!(result, 0b11111);
    }

    #[test]
    pub fn imm() {
        let opcode = 0b0101101010101010_1111111111111111_;
        let result = opcode::IMM.get(opcode);
        assert_eq!(result, 0b11111111_11111111);
    }

    #[test]
    pub fn rs() {
        let opcode = 0b101100_11111_011010101011110101110;
        let result = opcode::RS.get(opcode);
        assert_eq!(result, 0b11111);
    }
}
