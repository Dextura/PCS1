/// CPU State
pub struct Cpu {
    /// The Program Counter Register
    pc: u32,
}

impl Cpu {

    pub fn runt_next_intstruction(&mut self) {
        let pc = self.pc;

        /// Fetch Instruction At PC
        let instruction = self.load32(pc);

        /// Increment PC Point to the next instruction.
        self.pc = pc.wrapping_add(4);

        self.decode_and_execute(instruction);
    }
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            /// PC Reset Valua at start of bios
            pc: 0xbfc00000,
        }
    }
}
