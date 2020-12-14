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
    let mut bitmask: HashMap<usize, char> = input.lines().nth(0).unwrap().split("= ")
        .nth(1).unwrap().chars().enumerate().map(|(i,x)| (35 - i, x)).filter(|(_,x)| *x != 'X').collect();

    let mut iter = input.lines();
    iter.next();
    let mut mem: HashMap<u64, i64> = HashMap::new();

    for instruction in iter {
        if instruction.contains("mask") {
            bitmask = instruction.split("= ")
                .nth(1).unwrap().chars().enumerate().map(|(i,x)| (35 - i, x)).filter(|(_,x)| *x != 'X').collect();
        }
        else {
            let address = instruction.split("[").nth(1).unwrap().split("]").nth(0)
                .unwrap().parse::<u64>().unwrap();
            let mut value = instruction.split("= ").nth(1).unwrap().parse::<i64>().unwrap();
            let value_as_binary = to_binary(value);

            for (position, binary) in &bitmask {
                if value_as_binary.chars().nth(35 - *position).unwrap() != *binary {
                    if *binary == '0' {
                        value -= (2 as i64).pow(*position as u32);
                    }
                    else {
                        value += (2 as i64).pow(*position as u32);
                    }
                }
            }

            if !mem.contains_key(&address) {
                mem.insert(address, 0);
            }

            *mem.get_mut(&address).unwrap() = value;
        }
    }

    println!("Part 1: {}", mem.values().fold(0, |acc, x| acc + x));

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut bitmask = input.lines().nth(0).unwrap().split(" = ").nth(1).unwrap();
    let mut floating_bits: Vec<usize> = bitmask.chars().enumerate().filter(|(i,x)| *x == 'X').map(|(i,_)| i).collect();
    let mut num_floating_bits = floating_bits.len() as u32;
    let mut mem: HashMap<u64, i64> = HashMap::new();
    let mut iter = input.lines();
    iter.next();
    for line in iter {
        if line.contains("mask") {
            bitmask = line.split(" = ").nth(1).unwrap();
            floating_bits = bitmask.chars().enumerate().filter(|(i,x)| *x == 'X').map(|(i,_)| i).collect();
            num_floating_bits = floating_bits.len() as u32;
        }
        else {
            let mut address = to_binary(line.split("[").nth(1).unwrap().split("]").nth(0)
                .unwrap().parse::<i64>().unwrap());
            let mut value = line.split("= ").nth(1).unwrap().parse::<i64>().unwrap();
            let mut addresses: Vec<String> = Vec::new();
            let mut bits_to_override: Vec<usize> = Vec::new();
            for (i, bit) in bitmask.chars().enumerate() {
                if bit == '1' {
                    bits_to_override.push(i);
                }
            }
            for i in 0..(2 as i64).pow(num_floating_bits) {
                let as_binary = to_binary(i);
                let mut new_address = String::new();
                let mut binary_pointer = 0;
                for j in 0..36 {
                    if bits_to_override.contains(&j) {
                        new_address.push('1');
                    }
                    else if floating_bits.contains(&j) {
                        new_address.push(as_binary.chars().nth(35 - binary_pointer).unwrap());
                        binary_pointer += 1;
                    }
                    else {
                        new_address.push(address.chars().nth(j).unwrap());
                    }
                }
                addresses.push(new_address);
            }

            for address in &addresses {
                let address_decimal = to_decimal(address.clone());
                if !mem.contains_key(&address_decimal) {
                    mem.insert(address_decimal, 0);
                }
                *mem.get_mut(&address_decimal).unwrap() = value;
            }
        }
    }

    println!("Part 2: {}", mem.values().fold(0, |acc, x| acc + x));

    Ok(())
}

fn to_binary(decimal: i64) -> String {
    let mut binary = String::new();
    let mut dividend = decimal;

    for _ in 0..36 {
        if dividend <= 0 {
            binary.insert(0, '0');
        }
        else {
            binary.insert(0, std::char::from_digit((dividend % 2) as u32, 2).unwrap());
            dividend /= 2;
        }
    }

    return binary;
}

fn to_decimal(binary: String) -> u64 {
    binary.chars().rev().enumerate().fold(0, |acc,(i,x)| acc + (x.to_string().parse::<u64>().unwrap()) * (2 as u64).pow(i as u32))
}
