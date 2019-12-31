use std::fs;

fn main() {
    let filename = "./input.txt";
    let contents = fs::read_to_string(&filename).expect("error reading file");
    let lines = contents.lines();

    let mut total_fuel: u32 = 0;
    for line in lines {
        let mass: u32 = line.parse().expect("error parsing str -> int");

        let module_fuel = get_module_fuel(mass);
        total_fuel += module_fuel;
    }
    println!("total fuel mass needed: {}", total_fuel);
}

fn get_module_fuel(mass: u32) -> u32 {
    if (mass as i32 / 3) - 2 <= 0 {
        return 0; // return early if we would encounter an 'attempt to subtract with overflow' panic for uint
    }
    let mut module_fuel: u32 = (mass / 3) - 2;
    if module_fuel > 0 {
        module_fuel += get_module_fuel(module_fuel)
    }
    return module_fuel;
}
