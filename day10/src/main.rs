use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut numbers: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);
    let num_numbers = numbers.len();
    let mut one_jumps = 0;
    let mut three_jumps = 0;
    let mut pointer = 0;

    loop {
        if pointer >= num_numbers - 1 {
            break;
        }
        match numbers[pointer+1] - numbers[pointer] {
            3 => three_jumps += 1,
            2 => (),
            1 => one_jumps += 1,
            _ => break
        }
        pointer += 1;
    }

    println!("Part 1: {} times {} is {}", one_jumps, three_jumps, one_jumps * three_jumps);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut numbers: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    numbers.sort();
    numbers.insert(0, 0);
    let device_joltage = numbers.last().unwrap() + 3;
    numbers.push(device_joltage);
    let num_numbers = numbers.len();
    let mut num_paths_per_cell: HashMap<i64, i64> = numbers.iter().map(|x| (*x,0)).collect();
    *num_paths_per_cell.get_mut(&0).unwrap() = 1;

    let mut pointer = 0;
    loop {
        if pointer >= num_numbers {
            break;
        }
        let mut j = 0;
        loop {
            if pointer+j+1 < num_numbers && numbers[pointer+j+1] - numbers[pointer] <= 3 {
                *num_paths_per_cell.get_mut(&numbers[pointer+j+1])
                    .unwrap() += num_paths_per_cell[&numbers[pointer]];
            }
            else {
                break;
            }
            j += 1;
        }
        pointer += 1;
    }

    println!("Part 2: {}", num_paths_per_cell[&device_joltage]);

    Ok(())
}
