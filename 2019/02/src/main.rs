use std::fs;
extern crate intcode;

fn main() {
    let filename = "./input.txt";
    let mut intcodes: Vec<u32> = process_input(filename);

    intcode::process(&mut intcodes, Some(12), Some(2));
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
