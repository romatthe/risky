use std::{io, env};
use crate::cpu::Cpu;
use std::fs::File;
use std::io::Read;

mod cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: risky <filename>");
    }

    // Read the binary file into memory
    let mut file = File::open(&args[1])?;
    let mut binary = vec![];

    file.read_to_end(&mut binary)?;

    // Instantiate a new CPU
    let mut cpu = Cpu::new(binary);

    // Core fetch/decode/execute loop
    while cpu.pc < cpu.memory.len() as u64 {
        // Fetch
        let instruction = cpu.fetch();

        // Increment the program counter
        cpu.pc = cpu.pc + 4;

        // Decode

        // Execute
        cpu.execute(instruction);
    }

    Ok(())
}
