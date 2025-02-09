

struct CPU {
    
    r: [u32; 32],
    pc: u32,
    hi_lo: u32
    

}

impl CPU {

    fn cycle(&mut self){

        let instruction = self.load32(pc);

        self.pc = self.pc.wrapping_add(4);
        
        self.decode_and_execute(instruction);

    }

}