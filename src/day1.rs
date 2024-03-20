use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day1.txt").expect("Unable to read file");

    let mut sum = 0;
    let input = input.trim();
    input
        .chars()
        .into_iter()
        .map(|x| x.to_digit(10).unwrap())
        .tuple_windows::<(u32, u32)>()
        .for_each(|pair| {
            let a = pair.0;
            let b = pair.1;
            if a == b {
                sum += a;
            }
        });
    if input.chars().next().unwrap() == input.chars().last().unwrap() {
        sum += input.chars().next().unwrap().to_digit(10).unwrap();
    }
    println!("{}", sum);

    let mut sum = 0;
    for i in 0..input.len() / 2 {
        let a = input.chars().nth(i).unwrap().to_digit(10).unwrap();
        let b = input.chars().nth(i + input.len() / 2).unwrap().to_digit(10).unwrap();
        if a == b {
            sum += a*2;
        }
    }
    println!("{}", sum);
}
