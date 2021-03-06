use std::fs;
use intcode::IntcodeMachine;

fn read_input(path: &str) -> Vec<i64> {
    return fs::read_to_string(path)
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
}

fn main() {
    let tape: Vec<i64> = read_input("input");

    // Part 1
    let mut machine = IntcodeMachine::new(tape.clone())
        .with_input(1);

    let output = machine.run();
    println!("Part 1: {}", output[0]);

    // Part 2
    let mut machine = IntcodeMachine::new(tape)
        .with_input(5);

    let output = machine.run();
    println!("Part 2: {}", output[0]);
}
