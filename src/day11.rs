fn distance(x: i32, y: i32) -> i32 {
    let z = x.abs().min(y.abs());
    x.abs() + y.abs() - z.abs()
}


fn main() {
    let input = std::fs::read_to_string("data/day11.txt").expect("Unable to read file");

    let mut x = 0;
    let mut y = 0;
    let mut max_distance = 0;
    for direction in input.trim().split(',') {
        match direction {
            "n" => y += 1,
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => x += 1,
            "s" => y -= 1,
            "sw" => {
                x -= 1;
                y -= 1;
            }
            "nw" => x -= 1,
            _ => panic!("Unknown direction: {}", direction),
        }
        max_distance = std::cmp::max(max_distance, distance(x, y));
    }
    println!("X: {}, Y: {}", x, y);

    println!("Distance: {}", distance(x, y));
    println!("Max distance: {}", max_distance);
}