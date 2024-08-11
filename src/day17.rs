fn main() {
    let input = 301;
    let mut buffer = vec![0];
    let mut position = 0;
    for i in 1..2018 {
        position = (position + input) % buffer.len() + 1;
        buffer.insert(position, i);
    }
    println!("Part 1: {}", buffer[position + 1]);
    
    let mut after_zero = 0;
    position = 0;
    for i in 1..50_000_001 {
        position = (position + input) % i + 1;
        if position == 1 {
            after_zero = i;
        }
    }
    println!("Part 2: {}", after_zero);
}