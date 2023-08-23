use std::{collections::HashMap, fmt::Display};

const PLAYER_1_POS: usize = 7;
const PLAYER_2_POS: usize = 6;

const PART_TWO_SCORE: usize = 21;

fn next_die(deterministic_die: &mut usize) {
    *deterministic_die = (*deterministic_die + 1) % 100;
    if *deterministic_die == 0 {
        *deterministic_die = 100;
    }
}

fn deterministically_move_player(pos: usize, deterministic_die: &mut usize) -> usize {
    let mut new_pos = pos;
    for _ in 0..3 {
        next_die(deterministic_die);
        // println!("{} + {}", new_pos, deterministic_die);
        new_pos += *deterministic_die;
    }

    if new_pos % 10 == 0 {
        10
    } else {
        new_pos % 10
    }
}

fn p2_play_game(
    cache: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
    die_permutations: &Vec<usize>,
) -> (usize, usize) {
    if let Some((p1_unis, p2_unis)) = cache.get(&(p1_pos, p2_pos, p1_score, p2_score)) {
        return (*p1_unis, *p2_unis);
    }

    let mut p1_universes_for_this_state = 0;
    let mut p2_universes_for_this_state = 0;

    for p1_roll in die_permutations {
        for p2_roll in die_permutations {
            let mut p1_pos = (p1_pos + *p1_roll) % 10;
            if p1_pos == 0 {
                p1_pos = 10;
            }

            let p1_score = p1_score + p1_pos;

            let mut p2_pos = (p2_pos + *p2_roll) % 10;
            if p2_pos == 0 {
                p2_pos = 10;
            }

            let p2_score = p2_score + p2_pos;

            if p1_score >= PART_TWO_SCORE {
                p1_universes_for_this_state += 1;
                break;
            }

            if p2_score >= PART_TWO_SCORE {
                p2_universes_for_this_state += 1;
            } else {
                let (p1_sub_count, p2_sub_count) =
                    p2_play_game(cache, p1_pos, p2_pos, p1_score, p2_score, die_permutations);
                p1_universes_for_this_state += p1_sub_count;
                p2_universes_for_this_state += p2_sub_count;
            }
        }
    }

    cache.insert(
        (p1_pos, p2_pos, p1_score, p2_score),
        (p1_universes_for_this_state, p2_universes_for_this_state),
    );

    (p1_universes_for_this_state, p2_universes_for_this_state)
}

fn main() {
    let mut p1_pos = PLAYER_1_POS;
    let mut p1_score = 0;
    let mut p2_pos = PLAYER_2_POS;
    let mut p2_score = 0;
    let mut die_rolls = 0;
    let mut deterministic_die = 0;

    loop {
        p1_pos = deterministically_move_player(p1_pos, &mut deterministic_die);
        p1_score += p1_pos;
        die_rolls += 3;

        if p1_score >= 1000 {
            break;
        }

        p2_pos = deterministically_move_player(p2_pos, &mut deterministic_die);
        p2_score += p2_pos;
        die_rolls += 3;

        if p2_score >= 1000 {
            break;
        }
    }

    println!("{} {} {}", p1_score, p2_score, die_rolls);
    let score = p1_score.min(p2_score);
    println!("{}", score * die_rolls);

    let mut die_permutations = Vec::new();

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let sum = i + j + k;
                die_permutations.push(sum);
            }
        }
    }

    let mut part_two_cache = HashMap::new();

    let (p1_universes, p2_universes) = p2_play_game(
        &mut part_two_cache,
        PLAYER_1_POS,
        PLAYER_2_POS,
        0,
        0,
        &die_permutations,
    );

    println!(
        "{} {}, p1 won? {}",
        p1_universes,
        p2_universes,
        p1_universes > p2_universes
    );
}
