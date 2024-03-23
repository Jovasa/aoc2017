use std::fs;

fn main() {
    let input = fs::read_to_string("data/day5.txt").expect("Unable to read file");

    let mut registers = input.lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut steps = 0;
    let mut index = 0;
    while index < registers.len() {
        let jump = registers[index];
        if jump >= 3 {
            registers[index] -= 1;
        } else {
            registers[index] += 1;
        }
        index = (index as i32 + jump) as usize;
        steps += 1;
    }
    println!("{}", steps);
}