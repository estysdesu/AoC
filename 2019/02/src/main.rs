use std::fs;
extern crate intcode;

fn main() {
    let filename = "./input.txt";
    let orig_intcodes: Vec<u32> = process_input(filename);

    for n in 0..=99 {
        // noun
        for v in 0..=99 {
            // verb
            let mut intcodes = orig_intcodes.clone();
            intcode::process(&mut intcodes, Some(n), Some(v));
            let output = intcodes[0];
            if output == 19690720 {
                println!("{}", 100 * n + v)
            }
        }
    }
    // println!("{:?}", intcodes);
}

fn process_input(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("error reading file.");
    let content_vec: Vec<u32> = contents
        .split(",")
        .map(|m| m.parse().expect("error parsing str to int"))
        .collect();
    return content_vec;
}
