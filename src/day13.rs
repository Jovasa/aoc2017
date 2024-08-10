

fn main () {
    let input = std::fs::read_to_string("data/day13.txt").expect("Unable to read file");

    let mut firewall = std::collections::HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let depth = parts.next().unwrap().parse::<usize>().unwrap();
        let range = parts.next().unwrap().parse::<usize>().unwrap();
        firewall.insert(depth, range);
    }

    let mut severity = 0;
    for (depth, range) in &firewall {
        if depth % (2 * (range - 1)) == 0 {
            severity += depth * range;
        }
    }
    println!("Part 1: {}", severity);

    let mut delay = 0;
    while firewall.iter().any(|(depth, range)| (depth + delay) % (2 * (range - 1)) == 0) {
        delay += 1;
    }
    println!("Part 2: {}", delay);
}