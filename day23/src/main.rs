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
    let mut cups: Vec<u64> = input.lines().nth(0).unwrap()
        .chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut current_cup_pointer = 0;
    for i in 0..100 {
        println!("");
        println!("Move {}, current: {}, cp: {}", i+1, current_cup_pointer, cups[current_cup_pointer]);
        println!("cups: {:?}", cups);
        let mut current_cup_label = cups[current_cup_pointer];

        // Pick up three cups immediately clockwise of the current cup
        let mut three_cups: Vec<u64> = Vec::new();
        for j in 0..3 {
            three_cups.push(cups[(current_cup_pointer + 1 + j) % cups.len()]);
        }
        for cup in &three_cups {
            cups.remove(cups.iter().position(|x| x == cup).unwrap());
        }

        println!("pick up: {:?}", three_cups);
        //println!("cups: {:?}", cups);

        // Select destination cup
        let mut destination_label = current_cup_label;
        let mut destination_cup_pointer = 0;
        let cups_min_value = *cups.iter().min().unwrap();
        loop {
            destination_label -= 1;

            if cups.contains(&destination_label) {
                destination_cup_pointer = cups.iter().position(|x| *x == destination_label).unwrap();
                break;
            }
            if destination_label < cups_min_value {
                destination_cup_pointer = cups.iter().position(|x| x == cups.iter().max().unwrap()).unwrap();
                break;
            }
        }

        println!("destination: {}, ({})", cups[destination_cup_pointer], destination_cup_pointer);

        // Insert three cups clockwise to current
        if destination_cup_pointer == cups.len() - 1 {
            for cup in three_cups {
                cups.push(cup);
            }
        }
        else {
            for cup in three_cups.iter().rev() {
                cups.insert(destination_cup_pointer + 1, *cup);
            }
        }

        //println!("cups: {:?}", cups);

        current_cup_pointer = (cups.iter().position(|x| *x == current_cup_label).unwrap() + 1) % cups.len();
    }

    let mut pointer = cups.iter().position(|x| *x == 1).unwrap() + 1;
    let mut as_string = String::new();
    while as_string.len() < cups.len() - 1 {
        as_string.push_str(&*cups[pointer].to_string());
        pointer = (pointer + 1) % cups.len();
    }

    println!("Part 1: {}", as_string);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut cups: Vec<u64> = input.lines().nth(0).unwrap()
        .chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut successors: HashMap<u64, u64> = HashMap::new();
    for i in 0..cups.len() - 1 {
        successors.insert(cups[i], cups[i+1]);
    }
    successors.insert(*cups.last().unwrap(), cups[0]);
    let max_successor = cups.iter().max().unwrap();
    for i in max_successor+1..1000000+1 {
        successors.insert(i, i+1);
    }
    *successors.get_mut(cups.last().unwrap()).unwrap() = max_successor + 1;
    *successors.get_mut(&1000000).unwrap() = *cups.first().unwrap();

    let mut current = cups[0];
    for _ in 0..10000000 {
        let three_1 = successors[&current];
        let three_2 = successors[&three_1];
        let three_3 = successors[&three_2];

        let mut destination = current - 1;
        loop {
            if destination <= 0 {
                destination = *successors.values().max().unwrap();
            }
            if ![three_1, three_2, three_3].contains(&destination) {
                break;
            }
            destination -= 1;
        }

        *successors.get_mut(&current).unwrap() = successors[&three_3];
        *successors.get_mut(&three_3).unwrap() = successors[&destination];
        *successors.get_mut(&destination).unwrap() = three_1;

        current = successors[&current];
    }

    println!("Part 2: {}", successors[&1] * successors[&successors[&1]]);

    Ok(())
}
