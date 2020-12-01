use std::io::{self, Read};

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
    for a in 0..input_as_numbers.len() {
        let number_a = &input_as_numbers[a];

        for b in (a+1)..input_as_numbers.len() {
            let number_b = &input_as_numbers[b];

            if number_a + number_b == 2020  {
                return number_a * number_b;
            }
        }
    }

    return -1;
}

fn solve_part_2(input_as_numbers: &Vec<i32>) -> i32 {
    // Iterate over numbers and find the two that sum to 2020
    for a in 0..input_as_numbers.len() {
        let number_a = &input_as_numbers[a];

        for b in (a+1)..input_as_numbers.len() {
            let number_b = &input_as_numbers[b];

            for c in (b+1)..input_as_numbers.len() {
                let number_c = &input_as_numbers[c];
                if number_a + number_b + number_c == 2020  {
                    return number_a * number_b * number_c;
                }
            }
        }
    }

     return -1;
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input_as_numbers: Vec<i32> = get_input_as_numbers(&input);

    println!("Part 1: {}", solve_part_1(&input_as_numbers));
    println!("Part 2: {}", solve_part_2(&input_as_numbers));

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
