
const BIOS_START = 0xbfc00000;


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
    
    
    fn decode(opcode: u32){
           
    }

}

