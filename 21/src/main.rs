use std::fmt::Display;

const PLAYER_1_POS: usize = 4;
const PLAYER_2_POS: usize = 8;

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

#[derive(Clone)]
struct GameState {
    p1_pos: usize,
    p2_pos: usize,
    universe: usize,
    p1_score: usize,
    p2_score: usize,
    p1_roll_value: usize,
    p2_roll_value: usize,
}

impl GameState {
    fn new(p1_pos: usize, p2_pos: usize, p1_score: usize, p2_score: usize) -> Self {
        GameState {
            p1_pos,
            p2_pos,
            universe: 1,
            p1_score,
            p2_score,
            p1_roll_value: 0,
            p2_roll_value: 0,
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p1 pos {} roll {} score {}, p2 pos {} roll {} score {}, universe #{}",
            self.p1_pos,
            self.p1_roll_value,
            self.p1_score,
            self.p2_pos,
            self.p2_roll_value,
            self.p2_score,
            self.universe
        )
    }
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

    let mut work = vec![GameState::new(PLAYER_1_POS, PLAYER_2_POS, 0, 0)];

    let mut p1_universes = 0usize;
    let mut p2_universes = 0usize;

    while let Some(state) = work.pop() {
        for p1_roll in &die_permutations {
            for p2_roll in &die_permutations {
                let mut state = state.clone();
                state.p1_roll_value = *p1_roll;
                state.p1_pos = (state.p1_pos + *p1_roll) % 10;
                if state.p1_pos == 0 {
                    state.p1_pos = 10;
                }

                state.p1_score += state.p1_pos;

                state.p2_roll_value = *p2_roll;
                state.p2_pos = (state.p2_pos + *p2_roll) % 10;
                if state.p2_pos == 0 {
                    state.p2_pos = 10;
                }

                state.p2_score += state.p2_pos;

                if state.p1_score >= PART_TWO_SCORE {
                    p1_universes += state.universe;
                    break;
                }

                if state.p2_score >= PART_TWO_SCORE {
                    p2_universes += state.universe;
                } else {
                    work.push(state);
                }
            }
        }

        // for state in &work {
        //     println!("{}", state);
        // }
        // break;
    }

    println!(
        "{} {}, p1 won? {}",
        p1_universes,
        p2_universes,
        p1_universes > p2_universes
    );
}
