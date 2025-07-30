pub struct OpcodeRange(usize, usize);

/*
    Little-Endian bit table
    31-30-29-...-0
*/

impl OpcodeRange {
    pub fn get(&self, opcode: u32) -> u32 {
        let OpcodeRange(end, start) = self;

        let width = end - start + 1;
        let mask = (1 << width) - 1;

        (opcode >> start) & mask
    }


}

pub const PRIMARY: OpcodeRange = OpcodeRange(31, 26);
pub const RT: OpcodeRange = OpcodeRange(20, 16);
pub const IMM: OpcodeRange = OpcodeRange(15, 0);
pub const RS: OpcodeRange = OpcodeRange(25, 21);
