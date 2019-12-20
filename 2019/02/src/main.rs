use std::fs;

fn main() {
    let filename = "./input.txt";
    let mut intcodes: Vec<u32> = process_input(filename);

    // restore to state before crash
    intcodes[1] = 12; // modify pos 1 to value of 12
    intcodes[2] = 2; // modify pos 2 to value of 2

    intcode_processor(&mut intcodes);
    println!("{:?}", intcodes);
}

fn process_input(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("error reading file.");
    let content_vec: Vec<u32> = contents
        .split(",")
        .map(|m| m.parse().expect("error parsing str to int"))
        .collect();
    return content_vec;
}

fn intcode_processor(intcodes: &mut Vec<u32>) {
    const OPCODE_ADD: u32 = 1;
    const OPCODE_MULTIPLY: u32 = 2;
    const OPCODE_EXIT: u32 = 99;

    for i in 0..(intcodes.len() / 4) {
        let opcode = intcodes[i * 4];
        let n1_i = intcodes[(i * 4) + 1] as usize;
        let n2_i = intcodes[(i * 4) + 2] as usize;
        let output_i = intcodes[(i * 4) + 3] as usize;
        match opcode {
            OPCODE_ADD => intcodes[output_i] = intcodes[n1_i] + intcodes[n2_i],
            OPCODE_MULTIPLY => intcodes[output_i] = intcodes[n1_i] * intcodes[n2_i],
            OPCODE_EXIT => break,
            _ => panic!("invalid intcode sequence"),
        };
    }
}
