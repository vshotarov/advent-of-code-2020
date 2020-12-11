use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let (max_x, max_y) = (grid[0].len(), grid.len());

    loop {
        let mut new_grid = grid.clone();
        let mut modified = false;

        for x in 0..max_x {
            for y in 0..max_y {
                let cell = grid[y][x];
                if cell == '.' {
                    continue;
                }
                let mut num_occupied = 0;
                for dir_x in 0..3 {
                    for dir_y in 0..3 {
                        let nx = (x as i32 + dir_x - 1) as usize;
                        let ny = (y as i32 + dir_y - 1) as usize;
                        if x == nx && y == ny {
                            continue;
                        }
                        if nx < max_x && ny < max_y && grid[ny][nx] == '#' {
                            num_occupied += 1;
                        }
                    }
                }
                if cell == '#' && num_occupied >= 4 {
                    modified = true;
                    new_grid[y][x] = 'L';
                }
                else if cell == 'L' && num_occupied == 0 {
                    modified = true;
                    new_grid[y][x] = '#';
                }
            }
        }

        grid = new_grid;

        if !modified {
            break;
        }
    }

    let num_occupied = grid.iter().fold(0, |acc, x| acc + x.iter()
        .fold(0, |_acc, _x| _acc + match _x {
            '#' => 1,
            _ => 0
        }));

    println!("Part 1: {}", num_occupied);

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let (max_x, max_y) = (grid[0].len(), grid.len());

    loop {
        let mut new_grid = grid.clone();
        let mut modified = false;

        for x in 0..max_x {
            for y in 0..max_y {
                let cell = grid[y][x];
                if cell == '.' {
                    continue;
                }
                let mut num_occupied = 0;
                for dir_x in 0..3 {
                    for dir_y in 0..3 {
                        if dir_x == dir_y && dir_x == 1 {
                            continue;
                        }
                        let (mut nx, mut ny) = (x as i32, y as i32);
                        loop {
                            nx += dir_x - 1;
                            ny += dir_y - 1;
                            if !(nx >= 0 && nx < max_x as i32 && ny >= 0 && ny < max_y as i32) {
                                break;
                            }
                            if grid[ny as usize][nx as usize] == 'L' {
                                break;
                            }
                            else if grid[ny as usize][nx as usize] == '#' {
                                num_occupied += 1;
                                break;
                            }
                        }
                    }
                }
                if cell == '#' && num_occupied >= 5 {
                    modified = true;
                    new_grid[y][x] = 'L';
                }
                else if cell == 'L' && num_occupied == 0 {
                    modified = true;
                    new_grid[y][x] = '#';
                }
            }
        }

        grid = new_grid;

        if !modified {
            break;
        }
    }

    let num_occupied = grid.iter().fold(0, |acc, x| acc + x.iter()
        .fold(0, |_acc, _x| _acc + match _x {
            '#' => 1,
            _ => 0
        }));

    //println!("{}", grid.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
    println!("Part 2: {}", num_occupied);

    Ok(())
}
