fn main() {
    let input = 265149;
    manhattan_distance(input);

    let mut grid = vec![vec![0; 1000]; 1000];
    let mut x = 500;
    let mut y = 500;
    let mut dx: isize = 1;
    let mut dy: isize = 0;
    let mut sum = 1;
    let mut direction = 1;
    let mut total_steps: i32 = 2;
    let mut steps_to_direction = 1;
    grid[x][y] = sum;
    x += 1;
    grid[x][y] = sum;
    while sum < input {
        if steps_to_direction == ((total_steps as f64 ).sqrt()) as i32 {
            direction += 1;
            match direction % 4 {
                0 => {
                    dx = 0;
                    dy = 1;
                }
                3 => {
                    dx = -1;
                    dy = 0;
                }
                2 => {
                    dx = 0;
                    dy = -1;
                }
                1 => {
                    dx = 1;
                    dy = 0;
                }
                _ => {}
            }
            steps_to_direction = 0;
        }
        total_steps += 1;
        steps_to_direction += 1;
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
        sum = grid[x - 1][y - 1] + grid[x - 1][y] + grid[x - 1][y + 1] + grid[x][y - 1] + grid[x][y + 1] + grid[x + 1][y - 1] + grid[x + 1][y] + grid[x + 1][y + 1];
        grid[x][y] = sum;
    }
    println!("{}", sum);
}

fn manhattan_distance(input: i32) {
    let mut x = 1;
    while input > x * x {
        x += 2;
    }
    println!("{}", x - 1 - (x * x - input) % (x - 1));
}