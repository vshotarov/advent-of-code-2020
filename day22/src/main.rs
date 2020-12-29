use std::io::{self, Read};
use std::collections::{VecDeque, HashSet};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    solve_part1(&input)?;
    solve_part2(&input)?;

    Ok(())
}

fn solve_part1(input: &str) -> std::io::Result<()> {
    let mut player1_cards = input.split("\n\n").nth(0).unwrap().lines()
        .skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();
    let mut player2_cards = input.split("\n\n").nth(1).unwrap().lines()
        .skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();

    while player1_cards.len() > 0 && player2_cards.len() > 0 {
        let player1_card = player1_cards.pop_front().unwrap();
        let player2_card = player2_cards.pop_front().unwrap();
        if player1_card > player2_card {
            player1_cards.push_back(player1_card);
            player1_cards.push_back(player2_card);
        }
        else {
            player2_cards.push_back(player2_card);
            player2_cards.push_back(player1_card);
        }
    }

    let winner_cards = match player1_cards.len() {
        0 => player2_cards,
        _ => player1_cards
    };

    println!("Part 1: {}", winner_cards.iter().rev().enumerate()
        .fold(0, |acc,(i,x)| acc + x * (i as u64+1)));

    Ok(())
}

fn solve_part2(input: &str) -> std::io::Result<()> {
    let player1_cards = input.split("\n\n").nth(0).unwrap().lines()
        .skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();
    let player2_cards = input.split("\n\n").nth(1).unwrap().lines()
        .skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();

    let outcome = play_recursive_game(player1_cards, player2_cards, 0);
    let winner_cards = outcome.1;

    println!("Part 2: {}", winner_cards.iter().rev().enumerate()
        .fold(0, |acc,(i,x)| acc + x * (i as u64+1)));

    Ok(())
}

fn play_recursive_game(mut player1_cards: VecDeque<u64>, mut player2_cards: VecDeque<u64>, n: usize) -> (usize, VecDeque<u64>) {
    let mut iters = 0;
    let mut memory: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();
    //println!("{} {} {:?}, {:?}", "recursive game", n, player1_cards, player2_cards);
    while player1_cards.len() > 0 && player2_cards.len() > 0 {
        let player1_card = player1_cards.pop_front().unwrap();
        let player2_card = player2_cards.pop_front().unwrap();
        let mut winner = 0;

        let state = (player1_cards.clone(), player2_cards.clone());
        if memory.contains(&state) {
            //println!("game {}, ends with player 1 winning, because of a repeating state", n);
            return (0, player1_cards);
        }

        //println!("    p1 {}, p2 {}    {:?}, {:?}", player1_card, player2_card, player1_cards, player2_cards);
        if player1_card as usize <= player1_cards.len() &&
        player2_card as usize <= player2_cards.len() {
            let outcome = play_recursive_game(
                player1_cards.clone().iter().enumerate().filter(|(i,x)| *i<player1_card as usize).map(|(i,x)| *x).collect::<VecDeque<u64>>(),
                player2_cards.clone().iter().enumerate().filter(|(i,x)| *i<player2_card as usize).map(|(i,x)| *x).collect::<VecDeque<u64>>(),
                n+1);
            winner = outcome.0;
        }
        else if player1_card < player2_card {
            winner = 1;
        }

        if winner == 0 {
            player1_cards.push_back(player1_card);
            player1_cards.push_back(player2_card);
        }
        else {
            player2_cards.push_back(player2_card);
            player2_cards.push_back(player1_card);
        }

        memory.insert(state);
        iters += 1;
    }
    if player1_cards.len() > 0 {
        //println!("game {}, ends after {} iters with player {} winning", n, iters, 1);
        return (0, player1_cards);
    }
    else {
        //println!("game {}, ends after {} iters with player {} winning", n, iters, 2);
        return (1, player2_cards);
    }
}
