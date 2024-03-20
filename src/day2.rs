fn main() {
    let input = std::fs::read_to_string("data/day2.txt").expect("Unable to read file");

    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        sum += numbers.iter().max().unwrap() - numbers.iter().min().unwrap();
    }
    println!("{}", sum);

    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i != j && numbers[i] % numbers[j] == 0 {
                    sum += numbers[i] / numbers[j];
                }
            }
        }
    }
    println!("{}", sum);
}