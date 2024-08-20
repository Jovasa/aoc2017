use std::collections::HashMap;

fn get_value(registers: &HashMap<String, i64>, value: &str) -> i64 {
    if let Ok(number) = value.parse::<i64>() {
        number
    } else {
        *registers.get(value).unwrap_or(&0)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/day18.txt").expect("Unable to read file");
    let instructions: Vec<Vec<&str>> = input.lines().map(|x| x.split_whitespace().collect()).collect();

    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut last_sound = 0;
    let mut position = 0;
    loop {
        if position >= instructions.len() {
            break;
        }
        if do_one_instruction(&instructions, &mut registers, &mut last_sound, &mut position) { break; }
    }
}

fn do_one_instruction(instructions: &Vec<Vec<&str>>, registers: &mut HashMap<String, i64>, last_sound: &mut i64, position: &mut usize) -> bool {
    let instruction = &instructions[*position];
    match instruction[0] {
        "snd" => {
            *last_sound = get_value(&registers, instruction[1]);
        },
        "set" => {
            let value = get_value(&registers, instruction[2]);
            registers.insert(instruction[1].to_string(), value);
        },
        "add" => {
            let value = get_value(&registers, instruction[2]);
            *registers.entry(instruction[1].to_string()).or_insert(0) += value;
        },
        "mul" => {
            let value = get_value(&registers, instruction[2]);
            *registers.entry(instruction[1].to_string()).or_insert(0) *= value;
        },
        "mod" => {
            let value = get_value(&registers, instruction[2]);
            *registers.entry(instruction[1].to_string()).or_insert(0) %= value;
        },
        "rcv" => {
            if get_value(&registers, instruction[1]) != 0 {
                println!("Part 1: {}", last_sound);
                return true;
            }
        },
        "jgz" => {
            if get_value(&registers, instruction[1]) > 0 {
                *position = (*position as i64 + get_value(&registers, instruction[2]) - 1) as usize;
            }
        },
        _ => panic!("Unknown instruction"),
    }
    *position += 1;
    false
}