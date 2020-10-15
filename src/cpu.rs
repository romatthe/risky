// Integer register convention:
//
// Name 	ABI Mnemonic 	Meaning 	Preserved across calls?
// x0 	zero 	Zero 	-- (Immutable)
// x1 	ra 	Return address 	No
// x2 	sp 	Stack pointer 	Yes
// x3 	gp 	Global pointer 	-- (Unallocatable)
// x4 	tp 	Thread pointer 	-- (Unallocatable)
// x5-x7 	t0-t2 	Temporary registers 	No
// x8-x9 	s0-s1 	Callee-saved registers 	Yes
// x10-x17 	a0-a7 	Argument registers 	No
// x18-x27 	s2-s11 	Callee-saved registers 	Yes
// x28-x31 	t3-t6 	Temporary registers 	No

const MEMORY_SIZE: u64 = 1024 * 1024 * 128;
const REG_STACK_POINTER: usize = 0x02;

/// A RV64GC CPU
pub struct Cpu {
    /// The CPU's program counter
    pub pc: u64,
    /// 32 general-purpose registers, each 64-bits wide
    regs: [u64; 32],
    /// Memory directly accessible by the CPU
    pub memory: Vec<u8>,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        // Set the stack pointer to the size of a memory when a CPU is instantiated
        let mut regs = [0; 32];
        regs[REG_STACK_POINTER] = MEMORY_SIZE;

        Self {
            pc: 0,
            regs,
            memory: binary
        }
    }

    pub fn fetch(&self) -> u32 {
        // Read 32-bit instruction from memory
        unimplemented!()
    }

    pub fn execute(&mut self, instruction: u32) {
        // Decode an instruction and execute it
        unimplemented!()
    }
}

