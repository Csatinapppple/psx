fn main() {
    let mut cpu = CPU::new();
    let memory = vec![0; 4 * 1024 * 1024]; // Simulando 4MB de RAM
    
    let instruction = cpu.fetch(&memory);
    cpu.decode_execute(instruction);
}

