#[cfg(test)]
#[test]
fn test_process() {
    let mut intcodes = vec![2, 4, 4, 5, 99, 0];
    process(&mut intcodes, None, None);
    assert_eq!(intcodes[5], 9801);
}

pub fn process(code: &mut Vec<u32>, noun0: Option<u32>, verb0: Option<u32>) {
    // opcode definitions
    const OPCODE_ADD: u32 = 1; // addition
    const OPCODE_MULTIPLY: u32 = 2; // multiplation
    const OPCODE_EXIT: u32 = 99; // exit

    // replace first noun and first verb
    match noun0 {
        Some(n0) => code[1] = n0,
        None => (),
    }
    match verb0 {
        Some(v0) => code[2] = v0,
        None => (),
    }

    // loop calculate
    for i in 0..(code.len() / 4) {
        let j = i * 4; // real index
        let opcode = code[j];
        let p1_i = code[j + 1] as usize;
        let p2_i = code[j + 2] as usize;
        let p3_i = code[j + 3] as usize;
        match opcode {
            OPCODE_ADD => code[p3_i] = code[p1_i] + code[p2_i],
            OPCODE_MULTIPLY => code[p3_i] = code[p1_i] * code[p2_i],
            OPCODE_EXIT => break,
            _ => panic!("invalid opcode: {}", opcode),
        };
    }
}
