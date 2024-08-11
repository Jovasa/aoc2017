fn main() {
    let input = std::fs::read_to_string("data/day16.txt").expect("Unable to read file");
    let moves = input.trim().split(',').collect::<Vec<_>>();
    
    let mut programs = (b'a'..=b'p').collect::<Vec<_>>();
    let mut programs2 = programs.clone();
    
    let mut seen = std::collections::HashSet::new();
    let mut cycle = 0;
    for i in 0..1_000_000_000 {
        for m in &moves {
            match m.chars().next().unwrap() {
                's' => {
                    let n = m[1..].parse::<usize>().unwrap();
                    programs.rotate_right(n);
                }
                'x' => {
                    let mut parts = m[1..].split('/');
                    let a = parts.next().unwrap().parse::<usize>().unwrap();
                    let b = parts.next().unwrap().parse::<usize>().unwrap();
                    programs.swap(a, b);
                }
                'p' => {
                    let mut parts = m[1..].split('/');
                    let a = parts.next().unwrap().chars().next().unwrap();
                    let b = parts.next().unwrap().chars().next().unwrap();
                    let a = programs.iter().position(|&x| x == u8::try_from(a).unwrap()).unwrap();
                    let b = programs.iter().position(|&x| x == u8::try_from(b).unwrap()).unwrap();
                    programs.swap(a, b);
                }
                _ => panic!("Unknown move: {}", m),
            }
        }
        if i == 0 {
            println!("Part 1: {}", String::from_utf8(programs.clone()).unwrap());
        }
        if seen.contains(&programs) {
            cycle = i;
            break;
        }
        seen.insert(programs.clone());
    }
    
    for _ in 0..(1_000_000_000 % cycle) {
        for m in &moves {
            match m.chars().next().unwrap() {
                's' => {
                    let n = m[1..].parse::<usize>().unwrap();
                    programs2.rotate_right(n);
                }
                'x' => {
                    let mut parts = m[1..].split('/');
                    let a = parts.next().unwrap().parse::<usize>().unwrap();
                    let b = parts.next().unwrap().parse::<usize>().unwrap();
                    programs2.swap(a, b);
                }
                'p' => {
                    let mut parts = m[1..].split('/');
                    let a = parts.next().unwrap().chars().next().unwrap();
                    let b = parts.next().unwrap().chars().next().unwrap();
                    let a = programs2.iter().position(|&x| x == u8::try_from(a).unwrap()).unwrap();
                    let b = programs2.iter().position(|&x| x == u8::try_from(b).unwrap()).unwrap();
                    programs2.swap(a, b);
                }
                _ => panic!("Unknown move: {}", m),
            }
        }
    }
    println!("Part 2: {}", String::from_utf8(programs2).unwrap());
}