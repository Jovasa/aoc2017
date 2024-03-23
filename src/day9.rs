fn main() {
    let input = std::fs::read_to_string("data/day9.txt").expect("Unable to read file");

    let mut score = 0;
    let mut depth = 0;
    let mut garbage = false;
    let mut ignore = false;
    let mut garbage_count = 0;
    for c in input.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        if garbage {
            match c {
                '!' => ignore = true,
                '>' => garbage = false,
                _ => garbage_count += 1,
            }
        } else {
            match c {
                '{' => {
                    depth += 1;
                    score += depth;
                }
                '}' => depth -= 1,
                '<' => garbage = true,
                _ => (),
            }
        }
    }

    println!("Score: {}", score);
    println!("Garbage count: {}", garbage_count);
}