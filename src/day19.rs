fn main() {
    let input = std::fs::read_to_string("data/day19.txt").expect("Unable to read file");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let mut position: (usize, usize) = (0, grid[0].iter().position(|&x| x == '|').unwrap());
    let mut direction: (isize, isize) = (1, 0);
    let mut letters = String::new();
    let mut steps = 0;
    loop {
        position = ((position.0 as isize + direction.0) as usize, (position.1 as isize + direction.1) as usize);
        steps += 1;
        if position.0 < 0 || position.0 >= grid.len() || position.1 < 0 || position.1 >= grid[position.0].len() || grid[position.0][position.1] == ' ' {
            break;
        }
        let current = grid[position.0][position.1];
        if current == '+' {
            if direction.0 == 0 {
                if position.0 > 0 && grid[position.0 - 1][position.1] == '|' {
                    direction = (-1, 0);
                } else {
                    direction = (1, 0);
                }
            } else {
                if position.1 > 0 && grid[position.0][position.1 - 1] == '-' {
                    direction = (0, -1);
                } else {
                    direction = (0, 1);
                }
            }
        } else if current != ' ' && current != '|' && current != '-' {
            letters.push(current);
        }
    }
    println!("Part 1: {}", letters);
    println!("Part 2: {}", steps);
}