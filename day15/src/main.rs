use std::io::{self, Read};
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut numbers: VecDeque<u32> = VecDeque::new();
    for (i, number) in input.lines().nth(0).unwrap().split(",").enumerate() {
        numbers.push_front(number.parse::<u32>().unwrap());
    }
    let mut num_numbers = numbers.len();

    loop {
        if num_numbers == 2020 {
            println!("Part 1: {}", numbers[0]);
            break;
        }
        let mut iter = numbers.iter();
        iter.next();
        let position = match iter.position(|&x| x == numbers[0]) {
            Some(n) => n + 1,
            None => 0
        };
        numbers.push_front(position as u32);
        num_numbers += 1;
    }

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut numbers: HashMap<u64, u64> = HashMap::new();
    let mut last_number: u64 = 0;
    for (i, number) in input.lines().nth(0).unwrap().split(",").enumerate() {
        last_number = number.parse::<u64>().unwrap();
        numbers.insert(last_number, i as u64 + 1);
    }
    let mut num_numbers = numbers.len() as u64;
    numbers.remove(&last_number);

    loop {
        let number = match numbers.get(&last_number) {
            Some(n) => num_numbers - n,
            None => 0
        };

        if !numbers.contains_key(&last_number) {
            numbers.insert(last_number, num_numbers);
        }
        else {
            *numbers.get_mut(&last_number).unwrap() = num_numbers;
        }

        if num_numbers == 30000000 {
            println!("Part 2: {}", last_number);
            break;
        }

        num_numbers += 1;

        last_number = number;
    }

    Ok(())
}
