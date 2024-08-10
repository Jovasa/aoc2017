fn main() {
    let input = std::fs::read_to_string("data/day10.txt").expect("Unable to read file");

    let list_len: usize = 256;
    let mut list: Vec<i32> = (0..list_len as i32).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    let lengths = input.clone()
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    hash_round(list_len, &mut list, &mut current_position, &mut skip_size, lengths.clone());
    println!("Result: {}", list[0] * list[1]);

    let mut list2: Vec<i32> = (0..list_len as i32).collect();
    let mut lenghts2: Vec<usize> = input.trim().chars().map(|x| x as usize).collect();
    lenghts2.extend(vec![17, 31, 73, 47, 23]);
    current_position = 0;
    skip_size = 0;
    for _ in 0..64 {
        hash_round(list2.len(), &mut list2, &mut current_position, &mut skip_size, lenghts2.clone());
    }
    let mut dense_hash: Vec<i32> = Vec::new();
    for i in 0..16 {
        let mut value = 0;
        for j in 0..16 {
            value ^= list2[i * 16 + j];
        }
        dense_hash.push(value);
    }
    let result: String = dense_hash.iter().map(|x| format!("{:02x}", x)).collect();
    println!("Result: {}", result);
}


pub fn hash_round(list_len: usize, list: &mut Vec<i32>, current_position: &mut usize, skip_size: &mut usize, lengths: Vec<usize>) {
    for length in lengths {
        let mut sublist: Vec<i32> = Vec::new();
        for i in 0..length {
            sublist.push(list[(*current_position + i) % list_len]);
        }
        sublist.reverse();
        for i in 0..length {
            list[(*current_position + i) % list_len] = sublist[i];
        }
        *current_position += length + *skip_size;
        *current_position %= list_len;
        *skip_size += 1;
    }
}