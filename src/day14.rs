// include hash_round from day10.rs and use it to solve the puzzle.
mod day10;
use day10::hash_round;


fn mark_group(grid: &mut [[bool; 128]; 128], i: usize, j: usize) {
    if i < 128 && j < 128 && grid[i][j] {
        grid[i][j] = false;
        mark_group(grid, i + 1, j);
        mark_group(grid, i, j + 1);
        if i > 0 {
            mark_group(grid, i - 1, j);
        }
        if j > 0 {
            mark_group(grid, i, j - 1);
        }
    }
}


fn main() {
    let input = "hwlqcszp";

    let mut squares = 0;

    let mut grid = [[false; 128]; 128];

    for i in 0..128 {
        let lengths = format!("{}-{}", input, i);
        let mut list: Vec<i32> = (0..256).collect();
        let mut current_position = 0;
        let mut skip_size = 0;
        let mut lenghts2: Vec<usize> = lengths.trim().chars().map(|x| x as usize).collect();
        lenghts2.extend(vec![17, 31, 73, 47, 23]);
        for _ in 0..64 {
            hash_round(list.len(), &mut list, &mut current_position, &mut skip_size, lenghts2.clone());
        }
        let mut dense_hash: Vec<i32> = Vec::new();
        for i in 0..16 {
            let mut value = 0;
            for j in 0..16 {
                value ^= list[i * 16 + j];
            }
            dense_hash.push(value);
        }

        for (j, x) in dense_hash.iter().enumerate() {
            squares += x.count_ones();
            for k in 0..8 {
                grid[i][j * 8 + k] = x & (1 << (7 - k)) != 0;
            }
        }
    }
    println!("Part 1: {}", squares);

    let mut groups = 0;
    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] {
                groups += 1;
                mark_group(&mut grid, i, j);
            }
        }
    }
    println!("Part 2: {}", groups);
}
