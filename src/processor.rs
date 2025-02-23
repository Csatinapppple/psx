
let BIOS_START = 0xbfc00000;


struct CPU {
    r: [u32; 32], // 32 registradores de propósito geral
    pc: u32,              // Program Counter (PC)
    hi: u32,              // HI register (usado para multiplicações/divisões)
    lo: u32,              // LO register (usado para multiplicações/divisões)
}

impl CPU {
    fn new() -> Self {
        Self {
            r: [0; 32],
            pc: BIOS_START, // Endereço inicial do BIOS do PS1
            hi: 0,
            lo: 0,
        }
    }
    
    fn fetch(&self, memory: &[u8]) -> u32 {
        // Busca uma instrução da memória (4 bytes por instrução MIPS)
        let addr = self.pc as usize;
        u32::from_be_bytes([memory[addr], memory[addr + 1], memory[addr + 2], memory[addr + 3]])
    }
    
    fn decode_execute(&mut self, instruction: u32) {
        let opcode = (instruction >> 26) & 0x3F;
        match opcode {
            0x00 => self.execute_r_type(instruction), // Instruções R-type
            _ => println!("Opcode {:X} não implementado", opcode),
        }
    }
    
    fn execute_r_type(&mut self, instruction: u32) {
        let funct = instruction & 0x3F;
        match funct {
            0x20 => println!("ADD (soma) implementado futuramente"),
            _ => println!("Função {:X} não implementada", funct),
        }
    }

    
    fn cycle(&mut self, memory: &[u8]){
        
        let instruction = fetch(memory);

        decode_execute(instruction);

        self.pc += self.pc.wrapping_add(4);
        
    }

}

