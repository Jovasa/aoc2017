use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/day8.txt").expect("Unable to read file");

    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max_value = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let register = parts.next().unwrap().to_string();
        let operation = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().parse::<i32>().unwrap();
        parts.next(); // Skip "if"
        let condition_register = parts.next().unwrap().to_string();
        let condition = parts.next().unwrap().to_string();
        let condition_value = parts.next().unwrap().parse::<i32>().unwrap();

        let condition_register_value = *registers.get(&condition_register).unwrap_or(&0);
        let condition_result = match condition.as_str() {
            "==" => condition_register_value == condition_value,
            "!=" => condition_register_value != condition_value,
            "<" => condition_register_value < condition_value,
            "<=" => condition_register_value <= condition_value,
            ">" => condition_register_value > condition_value,
            ">=" => condition_register_value >= condition_value,
            _ => panic!("Unknown condition: {}", condition),
        };

        if condition_result {
            let register_value = *registers.get(&register).unwrap_or(&0);
            let new_value = match operation.as_str() {
                "inc" => register_value + value,
                "dec" => register_value - value,
                _ => panic!("Unknown operation: {}", operation),
            };
            max_value = std::cmp::max(max_value, new_value);
            registers.insert(register, new_value);
        }
    }

    let max = registers.values().max().unwrap();
    println!("Max register value: {}", max);
    println!("Max value: {}", max_value);
}