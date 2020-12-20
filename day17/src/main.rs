use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", solve(&input, 3));
    println!("Part 2: {}", solve(&input, 4));

    Ok(())
}

fn solve(input: &str, num_dimensions: u32) -> u32 {
    let mut lattice: HashMap<Vec<i32>, bool> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, cube) in line.chars().enumerate() {
            let mut key: Vec<i32> = vec![x as i32,y as i32];
            for _ in 0..num_dimensions - 2 {
                key.push(0);
            }
            lattice.insert(key, cube == '#');
        }
    }

    for _ in 0..6 {
        let mut lattice_copy = lattice.clone();
        for (key, _) in &lattice {
            for key_decimal in 0..(3 as u32).pow(num_dimensions) {
                let mut decimal = key_decimal as i32;
                let mut neighbour_key = key.clone();
                for i in 0..num_dimensions {
                    neighbour_key[(num_dimensions-1-i) as usize] += (decimal % 3) - 1;
                    decimal /= 3;
                }
                if !lattice_copy.contains_key(&neighbour_key) {
                    lattice_copy.insert(neighbour_key, false);
                }
            }
        }
        lattice = lattice_copy.clone();
        for (key, cube) in &lattice {
            let mut num_active_neighbours = 0;

            for key_decimal in 0..(3 as u32).pow(num_dimensions) {
                let mut decimal = key_decimal as i32;
                let mut neighbour_key = key.clone();
                for i in 0..num_dimensions {
                    neighbour_key[(num_dimensions-1-i) as usize] += (decimal % 3) - 1;
                    decimal /= 3;
                }
                if neighbour_key == *key {
                    continue;
                }
                if lattice.contains_key(&neighbour_key) && lattice[&neighbour_key] {
                    num_active_neighbours += 1;
                }
            }

            if *cube && !(num_active_neighbours == 2 || num_active_neighbours == 3) {
                *lattice_copy.get_mut(key).unwrap() = false;
            }
            if !*cube && num_active_neighbours == 3 {
                *lattice_copy.get_mut(key).unwrap() = true;
            }
        }
        lattice = lattice_copy;
    }

    return lattice.values().fold(0, |acc,x| acc + *x as u32);
}
