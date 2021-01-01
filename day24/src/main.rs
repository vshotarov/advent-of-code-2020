use std::io::{self, Read};
use std::collections::{HashSet, HashMap};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tiles = solve_part1(&input)?;
    solve_part2(tiles)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<HashSet<(i32,i32)>> {
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        let mut this_tile = (0,0);
        let mut prev = 'f';
        for x in line.chars() {
            match x {
                'e' => { if "sn".contains(prev) { this_tile.0 += 1; } else { this_tile.0 += 2; } },
                'w' => { if "sn".contains(prev) { this_tile.0 -= 1; } else { this_tile.0 -= 2; } },
                's' => this_tile.1 -= 1,
                'n' => this_tile.1 += 1,
                _ => panic!("unexpected char {}", x)
            }
            prev = x;
        }

        if tiles.contains(&this_tile) {
            tiles.take(&this_tile);
        }
        else {
            tiles.insert(this_tile);
        }
    }

    println!("Part 1: {}", tiles.len());
    
    Ok(tiles)
}

fn solve_part2(mut tiles: HashSet<(i32,i32)>) -> std::io::Result<()> {
    for _ in 0..100 {
        let mut neighbours:HashMap<(i32,i32), u8> = HashMap::new();
        for (x,y) in tiles.iter() {
            for (dx,dy) in [(0,0),(2,0),(-2,0),(-1,1),(1,1),(1,-1),(-1,-1)].iter() {
                *neighbours.entry((*x + dx, *y + dy)).or_insert(0) += 1;
            }
        }

        tiles = neighbours.iter().filter(
                |(coords, num_black_neighbours)| **num_black_neighbours == 2 || **num_black_neighbours == 3 && tiles.contains(coords))
            .map(|(coords, _)| *coords)
            .collect();
    }

    println!("Part 2: {}", tiles.len());

    Ok(())
}
