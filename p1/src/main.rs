use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(&filename).expect("Error reading file.");
    let lines = contents.lines();

    let mut fuel: u32 = 0;
    for line in lines {
        let mass: u32 = line.parse().expect("Error parsing str -> int");
        fuel += mod_fuel_req(mass);
    }
    println!("{}", fuel)
}

fn mod_fuel_req(mass: u32) -> u32 {
    let mod_fuel: u32 = (mass / 3) - 2;
    return mod_fuel;
}
