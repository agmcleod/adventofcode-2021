const PLAYER_1_POS: usize = 7;
const PLAYER_2_POS: usize = 6;

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
        // println!(
        //     "p1 pos {} score {} final die {}",
        //     p1_pos, p1_score, deterministic_die
        // );
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
}
