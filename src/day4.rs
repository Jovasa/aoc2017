use std::fs;

fn main() {
    let input = fs::read_to_string("data/day4.txt").expect("Unable to read file");

    let mut valids = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let unqiue_words = words.iter().collect::<std::collections::HashSet<_>>();
        let sorted_uniq_words = unqiue_words.iter().map(|x| {
            let mut chars: Vec<char> = x.chars().collect();
            chars.sort();
            chars.iter().collect::<String>()
        }).collect::<std::collections::HashSet<_>>();
        if words.len() == unqiue_words.len() && words.len() == sorted_uniq_words.len() {
            valids += 1;
        }
    }
    println!("{}", valids);
}