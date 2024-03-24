fn main() {
    let input = std::fs::read_to_string("data/day12.txt").expect("Unable to read file");

    let mut graph = std::collections::HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" <-> ");
        let node = parts.next().unwrap().parse::<usize>().unwrap();
        let neighbors = parts.next().unwrap().split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        graph.insert(node, neighbors);
    }


    let mut visited = std::collections::HashSet::new();
    let mut not_visited = graph.keys().cloned().collect::<std::collections::HashSet<_>>();
    let mut groups = 0;
    while visited.len() < graph.len(){
        let mut next = vec![*not_visited.iter().next().unwrap()];
        groups += 1;
        while !next.is_empty() {
            let node = next.pop().unwrap();
            visited.insert(node);
            not_visited.remove(&node);
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        next.push(*neighbor);
                    }
                }
            }
        }
        println!("Part 1: {}", visited.len());
    }
    println!("Part 2: {}", groups);
}