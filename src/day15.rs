fn main() {
    let input = std::fs::read_to_string("data/day15.txt").expect("Unable to read file");
    let mut lines = input.lines();
    let a_original: u64 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let b_original: u64 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    
    let mut a = a_original;
    let mut b = b_original;
    
    let a_factor = 16807;
    let b_factor = 48271;
    
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = (a * a_factor) % 2147483647;
        b = (b * b_factor) % 2147483647;
        if a & 0xFFFF == b & 0xFFFF {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
    
    let mut a = a_original;
    let mut b = b_original;
    
    let mut count = 0;
    for _ in 0..5_000_000 {
        loop {
            a = (a * a_factor) % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (b * b_factor) % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }
        if a & 0xFFFF == b & 0xFFFF {
            count += 1;
        }
    }
    println!("Part 2: {}", count);
}