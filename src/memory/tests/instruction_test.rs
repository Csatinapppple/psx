#[cfg(test)]
mod instruction_test {

    use crate::memory::instruction::Instruction;

    #[test]
    pub fn test_primary() {
        let instruction = Instruction(0b11111100_00000000_00000000_00000000);
        let result = instruction.primary();

        assert_eq!(result, 0b111111);
    }

    #[test]
    pub fn test_rt() {
        let instruction = Instruction(0b00000000_00011111_00000000_00000000);
        let result = instruction.rt();
        assert_eq!(result, 0b1_1111);
    }

    #[test]
    pub fn test_imm() {
        let instruction = Instruction(0b00000000_00000000_11111111_11111111);
        let result = instruction.imm();
        assert_eq!(result, 0xffff);
    }

    #[test]
    pub fn test_rs() {
        let instruction = Instruction(0b00000011_11100000_00000000_00000000);
        let result = instruction.rs();
        assert_eq!(result, 0b1_1111);
    }
}
