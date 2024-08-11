use std::collections::HashMap;

struct Program {
    children: Vec<String>,
    parent: Option<String>,
    weight: i32,
}


fn print_weights_of_childern(programs: &HashMap<String, Program>, name: &String, depth: i32) {
    let program = programs.get(name).unwrap();
    for _ in 0..depth {
        print!(" ");
    }
    println!("{}: {}", name, get_weight(programs, name));
    for child in &program.children {
        print_weights_of_childern(programs, child, depth + 1);
    }
}


fn get_weight(programs: &HashMap<String, Program>, name: &String) -> i32 {
    let program = programs.get(name).unwrap();
    let mut weight = program.weight;
    let mut weights = Vec::new();
    for child in &program.children {
        let temp = get_weight(programs, child);
        weights.push((temp, child));
        weight += temp;
    }

    if weights.len() > 1 {
        let max = weights.iter().max().unwrap().0;
        let min = weights.iter().min().unwrap().0;
        if max != min {
            let max_name = weights.iter().find(|x| x.0 == max).unwrap().1;
            let min_name = weights.iter().find(|x| x.0 == min).unwrap().1;
            println!("{}: {} {} {}", name, weight, max_name, min_name);
            print_weights_of_childern(programs, max_name, 0);
            print_weights_of_childern(programs, min_name, 0);
        }
    }


    weight
}

fn main() {
    let input = std::fs::read_to_string("data/day7.txt").expect("Unable to read file");

    let mut programs: HashMap<String, Program> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let weight = parts.next().unwrap().trim_matches(|x| x == '(' || x == ')').parse::<i32>().unwrap();
        let children = parts.skip(1).map(|x| x.trim_matches(',').to_string()).collect
            ::<Vec<String>>();

        if programs.contains_key(&name) {
            programs.get_mut(&name).unwrap().children = children.clone();
            programs.get_mut(&name).unwrap().weight = weight;
        }
        else {
            let parent = if let Some(x) = programs.get(&name) {
                x.parent.clone()
            } else {
                None
            };

            programs.insert(name.clone(), Program {
                children: children.clone(),
                parent,
                weight,
            });
        }

        for child in children {
            if let Some(x) = programs.get_mut(&child) {
                x.parent = Some(name.clone());
            } else {
                programs.insert(child.clone(), Program {
                    children: Vec::new(),
                    parent: Some(name.clone()),
                    weight: 0,
                });
            }
        }
    }

    let root_name = programs.iter().find(|&(_, x)| x.parent.is_none()).unwrap().0;
    println!("{}", root_name);
    get_weight(&programs, root_name);
}