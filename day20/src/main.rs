use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug,Clone)]
struct Tile {
    id: String,
    data: Vec<String>
}

impl Tile {
    fn get_borders(&mut self) -> Vec<String> {
        let top = self.data[0].clone();;
        let right = self.data.iter()
            .map(|x| x[9..10].to_string()).collect::<Vec<String>>()
            .join("");
        let bottom = self.data[9].clone();;
        let left = self.data.iter()
            .map(|x| x[0..1].to_string()).collect::<Vec<String>>()
            .join("");
        vec![top, right, bottom, left]
    }

    fn flip_x(&mut self) {
        self.data = self.data.iter().map(|x| x.chars().rev().collect::<String>()).collect();
    }

    fn flip_y(&mut self) {
        self.data.reverse();
    }

    fn rotate(&mut self) {
        let mut data = vec![vec![] as Vec<char>; self.data.len()];
        for x in self.data.iter() {
            for i in 0..self.data.len() {
                data[i].push(x.chars().nth(self.data.len()-1-i).unwrap());
            }
        }
        self.data = data.iter().map(|x| x.iter().collect::<String>()).collect();
    }

    fn strip_borders(&mut self) {
        self.data = self.data.iter().skip(1).rev().skip(1).rev().map(
            |x| x[1..9].to_string()).collect();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile id: {}\n{}", self.id, self.data.join("\n"))
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tiles = parse_input(&input);

    let corner_tile_ids = solve_part1(&mut tiles)?;
    solve_part2(&mut tiles, &corner_tile_ids[0])?;

    Ok(())
}

fn solve_part1(tiles: &mut HashMap<&str, Tile>) -> std::io::Result<Vec<String>> {
    let mut corner_product = 1;
    let mut corner_tile_ids: Vec<String> = Vec::new();

    for (id, tile) in tiles.clone().iter_mut() {
        let tile_borders = tile.get_borders();
        let tile_flipped_borders = tile_borders.iter().rev().map(|x|
            x.chars().rev().collect::<String>()).collect::<Vec<String>>();
        let mut matches = 0;
        for (other_id, other_tile) in tiles.iter_mut() {
            if other_id == id {
                continue;
            }
            let other_tile_borders = other_tile.get_borders();
            for border in other_tile_borders {
                if tile_borders.contains(&border) || tile_flipped_borders.contains(&border) {
                    matches += 1;
                    break;
                }
            }
        }
        if matches == 2 {
            corner_product *= id.parse::<u64>().unwrap();
            corner_tile_ids.push(id.to_string());
        }
    }

    println!("Part 1: {}", corner_product);

    Ok(corner_tile_ids)
}

fn solve_part2(tiles: &mut HashMap<&str, Tile>, corner_tile_id: &String) -> std::io::Result<()> {
    let corner_tile_borders = tiles.get_mut(&corner_tile_id[..]).unwrap().get_borders();
    let mut corner_tile_matching_borders = vec![false, false, false, false];
    for (id, tile) in tiles.iter_mut() {
        if id == corner_tile_id {
            continue;
        }
        let mut other_tile_all_borders = tile.get_borders();
        other_tile_all_borders.append(&mut other_tile_all_borders.iter().rev()
            .map(|x| x.chars().rev().collect::<String>()).collect());
        for i in 0..4 {
            if other_tile_all_borders.contains(&corner_tile_borders[i]) {
                corner_tile_matching_borders[i] = true;
            }
        }
        if corner_tile_matching_borders.iter().filter(|&&x| x).count() == 2 {
            break;
        }
    }

    while corner_tile_matching_borders != vec![false, true, true, false] {
        let mut new_corner_tile_matching_borders = Vec::new();
        new_corner_tile_matching_borders.push(corner_tile_matching_borders[1]);
        new_corner_tile_matching_borders.push(corner_tile_matching_borders[2]);
        new_corner_tile_matching_borders.push(corner_tile_matching_borders[3]);
        new_corner_tile_matching_borders.push(corner_tile_matching_borders[0]);
        corner_tile_matching_borders = new_corner_tile_matching_borders;
        tiles.get_mut(&corner_tile_id[..]).unwrap().rotate();
    }

    let num_rows = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec!["".to_string(); num_rows]; num_rows];
    grid[0][0] = corner_tile_id.to_string();
    let mut taken_ids = vec![corner_tile_id.to_string()];

    for row in 0..num_rows {
        if row != 0 {
            let tile_above_borders = tiles.get_mut(&grid[row-1][0][..]).unwrap().get_borders();;
            let border_above = tile_above_borders.get(2).unwrap();
            let mut found = false;
            let mut found_id = "".to_string();
            for (id, tile) in tiles.iter_mut() {
                if taken_ids.contains(&id.to_string()) {
                    continue;
                }
                let tile_borders = tile.get_borders();
                let tile_flipped_borders = tile_borders.iter().rev().map(|x|
                    x.chars().rev().collect::<String>()).collect::<Vec<String>>();
                if tile_borders.contains(border_above) || tile_flipped_borders.contains(border_above) {
                    let mut side = tile_borders.iter().position(|x| x == border_above).unwrap_or(5);
                    if side == 5 {
                        tile.flip_x();
                        tile.flip_y();
                        side = tile.get_borders().iter().position(|x| x == border_above).unwrap();
                    }
                    match side {
                        0 => (),
                        1 => tile.rotate(),
                        2 => tile.flip_y(),
                        3 => { tile.flip_x(); tile.rotate() },
                        _ => panic!("unrecognized side {}", side)
                    }
                    found = true;
                    found_id = id.to_string();
                    break;
                }
            }
            if !found {
                panic!("row not found");
            }
            taken_ids.push(found_id.clone());
            grid[row][0] = found_id;
        }

        for col in 1..num_rows {
            let tile_to_the_left_borders = tiles.get_mut(&grid[row][col-1][..]).unwrap().get_borders();;
            let border_to_the_left = tile_to_the_left_borders.get(1).unwrap();
            let mut found = false;
            let mut found_id = "".to_string();
            for (id, tile) in tiles.iter_mut() {
                if taken_ids.contains(&id.to_string()) {
                    continue;
                }
                let tile_borders = tile.get_borders();
                let tile_flipped_borders = tile_borders.iter().rev().map(|x|
                    x.chars().rev().collect::<String>()).collect::<Vec<String>>();
                if tile_borders.contains(border_to_the_left) || tile_flipped_borders.contains(border_to_the_left) {
                    let mut side = tile_borders.iter().position(|x| x == border_to_the_left).unwrap_or(5);
                    if side == 5 {
                        tile.flip_x();
                        tile.flip_y();
                        side = tile.get_borders().iter().position(|x| x == border_to_the_left).unwrap();
                    }
                    match side {
                        0 => { tile.rotate(); tile.flip_y(); },
                        1 => tile.flip_x(),
                        2 => { tile.flip_y(); tile.rotate(); tile.flip_y(); },
                        3 => (),
                        _ => panic!("unrecognized side {}", side)
                    }
                    found = true;
                    found_id = id.to_string();
                    break;
                }
            }
            if !found {
                panic!("row not found");
            }
            taken_ids.push(found_id.clone());
            grid[row][col] = found_id;
        }
    }

    let mut image = vec!["".to_string(); num_rows * 8];
    for (row, cols) in grid.iter().enumerate() {
        for (col, tile_id) in cols.iter().enumerate() {
            tiles.get_mut(&tile_id[..]).unwrap().strip_borders();
            for i in 0..8 {
                image.get_mut(row*8+i)
                    .unwrap().push_str(&tiles.get_mut(&tile_id[..]).unwrap().data[i][..]);
            }
        }
    }

    let mut image_tile = Tile{id: "0".to_string(), data: image};

    let monster = "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   "[1..].lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let mut monster_bits: Vec<(usize, usize)> = Vec::new();
    for row in 0..3 {
        for (col, bit) in monster[row].chars().enumerate() {
            if bit == '#' {
                monster_bits.push((row, col));
            }
        }
    }

    let mut num_monsters = 0;
    let mut num_rotations = 0;
    let mut monster_hashes: HashSet<(usize, usize)> = HashSet::new();
    while num_monsters == 0 {
        if num_rotations == 4 {
            image_tile.flip_x();
        }
        if num_rotations == 9 {
            panic!("Can't find any bloody monsters");
        }
        for row in 0..(image_tile.data.len() - 3) {
            'outer: for col in 0..(image_tile.data[0].len() - monster[0].len()) {
                let mut potential_monster_hashes: HashSet<(usize, usize)> = HashSet::new();
                for (y,x) in &monster_bits {
                    if image_tile.data[row+y].chars().nth(col+x).unwrap() == '#' {
                        potential_monster_hashes.insert((row+y, col+x));
                    }
                    else {
                        continue 'outer;
                    }
                }
                let new: HashSet<_> = monster_hashes.union(&potential_monster_hashes).collect();
                monster_hashes = new.iter().map(|x| **x).collect();
                num_monsters += 1;
            }
        }
        if num_monsters == 0 {
            image_tile.rotate();
            num_rotations += 1;
        }
    }

    image_tile.data = image_tile.data.iter().enumerate()
        .map(|(i,line)| line.chars().enumerate()
            .map(|(j,x)| {
                if monster_hashes.contains(&(i,j)) {
                    '0'
                }
                else {
                    x
                }
            }).collect::<String>()).collect::<Vec<String>>();

    let mut roughness = 0;
    for (row, chars) in image_tile.data.iter().enumerate() {
        for (col, x) in chars.chars().enumerate() {
            if x == '#' {
                roughness += 1;
            }
        }
    }

    println!("Part 2: {}", roughness);
    
    Ok(())
}

fn parse_input(input: &str) -> HashMap::<&str, Tile> {
    let mut tiles: HashMap<&str, Tile> = HashMap::new();
    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = lines.nth(0).unwrap().split(" ").nth(1).unwrap()
            .split(":").nth(0).unwrap();
        tiles.insert(id, Tile {
            id: id.to_string(),
            data: lines.map(|x| x.to_string()).collect()
        });
    }
    tiles
}
