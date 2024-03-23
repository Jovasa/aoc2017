use std::fs;

fn main() {
    let input = fs::read_to_string("data/day6.txt").expect("Unable to read file");

    let mut banks = input.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut seen = std::collections::HashSet::new();
    let mut cycles = 0;
    let mut order_of_seen = Vec::new();
    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        order_of_seen.push(banks.clone());
        let max = banks.iter().max().unwrap();
        let mut index = banks.iter().position(|x| x == max).unwrap();
        let mut blocks = *max;
        banks[index] = 0;
        while blocks > 0 {
            index = (index + 1) % banks.len();
            banks[index] += 1;
            blocks -= 1;
        }
        cycles += 1;
    }
    let first_seen = order_of_seen.iter().position(|x| x == &banks).unwrap();
    println!("{}", cycles);
    println!("{}", cycles - first_seen);
}