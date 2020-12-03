use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;

    let input: Vec<&str> = raw_input.lines().collect();

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &Vec<&str>) -> std::io::Result<()> {
    let rows = input.len();
    let columns = input[0].len();

    let (mut x, mut y) = (0, 0);
    let mut num_trees = 0;

    loop {
        if input[y].chars().nth(x) == Some('#') {
            num_trees += 1;
        }

        x = (x+3) % columns;
        y += 1;

        if y >= rows {
            break;
        }
    }

    println!("Part 1: num trees {}", num_trees);

    Ok(())
}

fn solve_part2(input: &Vec<&str>) -> std::io::Result<()> {
    let rows = input.len() as u32;
    let columns = input[0].len() as u32;

    let slopes: [(u32, u32); 5] = [(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut coords: [(u32, u32); 5] = [(0,0);5];
    let mut finished: [bool; 5] = [false; 5];
    let mut num_trees: [u64; 5] = [0; 5];

    loop {
        for i in 0..5 {
            if finished[i] { continue; }

            if input[coords[i].1 as usize].chars().nth(coords[i].0 as usize) == Some('#') {
                num_trees[i] += 1;
            }

            coords[i].0 = (coords[i].0 + slopes[i].0) % columns;
            coords[i].1 += slopes[i].1;

            if coords[i].1 >= rows {
                finished[i] = true;
            }
        }

        if finished.iter().find(|&&x| x == false) == None {
            break;
        }
    }

    println!("Part2: Product of trees {}",
        num_trees.iter().fold(1, |product, x| product * x));

    Ok(())
}
