use std::collections::{HashMap, VecDeque};

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

    let mut registers0: HashMap<String, i64> = HashMap::new();
    registers0.insert("p".to_string(), 0);
    let mut sends = 0;
    let mut position0 = 0;
    let mut out_stack0: VecDeque<i64> = VecDeque::new();

    let mut registers1: HashMap<String, i64> = HashMap::new();
    registers1.insert("p".to_string(), 1);
    let mut position1 = 0;
    let mut out_stack1: VecDeque<i64> = VecDeque::new();

    loop {
        if position0 >= instructions.len() || position1 >= instructions.len() {
            break;
        }
        if instructions[position1][0] == "snd" {
            sends += 1;
        }
        if let Some(value) = do_one_instruction(&instructions, &mut registers0, &mut out_stack1, &mut position0) {
            out_stack0.push_back(value);
        }
        if let Some(value) = do_one_instruction(&instructions, &mut registers1, &mut out_stack0, &mut position1) {
            out_stack1.push_back(value);
        }
        if instructions[position0][0] == "rcv" && instructions[position1][0] == "rcv" && out_stack0.is_empty() && out_stack1.is_empty() {
            break;
        }
    }
    println!("Part 2: {}", sends);
}

fn do_one_instruction(instructions: &Vec<Vec<&str>>, registers: &mut HashMap<String, i64>, last_sound: &mut VecDeque<i64>, position: &mut usize) -> Option<i64> {
    let instruction = &instructions[*position];
    match instruction[0] {
        "snd" => {
            let temp =  get_value(&registers, instruction[1]);
            *position += 1;
            return Some(temp);
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
            if let Some(value) = last_sound.pop_front() {
                registers.insert(instruction[1].to_string(), value);
            } else {
                return None;
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
    None
}