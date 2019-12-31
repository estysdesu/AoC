use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "./input.txt";

// Setup for parsing
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Length = u32;
struct Instruction {
    dir: Direction,
    len: Length,
}
impl Instruction {
    fn new(d: Direction, l: Length) -> Self {
        Instruction { dir: d, len: l }
    }
}

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let (v1_string, v2_string) = parse_input(&FILENAME);
}

fn parse_input(filename: &str) -> (Vec<&str>, Vec<&str>) {
    let contents = fs::read_to_string(filename).expect("error reading file.");
    let contents_split: Vec<Vec<&str>> = contents.lines().map(|v| v.split(",").collect()).collect();

    return (contents_split[0], contents_split[1]);
}
