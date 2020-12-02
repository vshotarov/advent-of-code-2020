use std::io::{self, Read};
use std::collections::HashSet;
use std::time::Instant;


fn get_input_as_numbers(input: &str) -> Vec<i32> {
    let input_as_lines: Vec<&str> = input.lines().collect();
    let mut input_as_numbers: Vec<i32> = Vec::new();

    // Convert lines into numbers
    for line in input_as_lines {
        input_as_numbers.push(line.parse().unwrap());
    }

    return input_as_numbers;
}

fn solve_part_1(input_as_numbers: &Vec<i32>) -> i32 {
    // Iterate over numbers and find the two that sum to 2020
    let mut hash_set: HashSet<i32> = HashSet::new();
    let target: i32 = 2020;

    for i in 0..input_as_numbers.len() {
        let num: i32 = input_as_numbers[i];
        let diff: i32 = target - num;

        if hash_set.contains(&diff) {
            return diff * num;
        }

        hash_set.insert(num);
    }

    return -1;
}

fn solve_part_2(input_as_numbers: &Vec<i32>) -> i32 {
    // Iterate over numbers and find the two that sum to 2020
    let target: i32 = 2020;

    for i in 0..input_as_numbers.len() {
        let num_a: i32 = input_as_numbers[i];
        let mut hash_set: HashSet<i32> = HashSet::new();
        let inbetween_target: i32 = target - num_a;

        for j in i..input_as_numbers.len() {
            let num_b: i32 = input_as_numbers[j];
            let diff: i32 = inbetween_target - num_b;

            if hash_set.contains(&diff) {
                return diff * num_a * num_b;
            }

            hash_set.insert(num_b);
        }
    }

    return -1;
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input_as_numbers: Vec<i32> = get_input_as_numbers(&input);

    let before_part1 = Instant::now();
    println!("Part 1: {}", solve_part_1(&input_as_numbers));
    println!("Part 1 took {:.2?}", before_part1.elapsed());
    let before_part2 = Instant::now();
    println!("Part 2: {}", solve_part_2(&input_as_numbers));
    println!("Part 2 took {:.2?}", before_part2.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Allows accessing functions from outside the `tests` scope

    #[test]
    fn test_part1_on_example_input() {
        let example_input = std::fs::read_to_string("input/example_input.txt").unwrap();
        let example_as_numbers = get_input_as_numbers(&example_input);
        assert_eq!(solve_part_1(&example_as_numbers), 514579);
    }
}
