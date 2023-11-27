use std::collections::HashSet;

type BadState = HashSet<(usize, i32)>;

// borrowed solution from: https://github.com/VSZM/Advent_Of_Code/blob/master/2021/AOC2021/Day24.cs, as hints on reddit werent the most clear as to the patterns to look at
fn run_sub_program(
    inputs: &Vec<i32>,
    bad_states: &mut BadState,
    mut model_number: usize,
    z: i32,
    depth: usize,
    divs: &Vec<i32>,
    adds: &Vec<i32>,
    second_adds: &Vec<i32>,
) -> Option<usize> {
    if bad_states.contains(&(depth, z)) || depth == 14 {
        return None;
    }

    // this is to shift the number left, so were adding to the next place in the number
    model_number *= 10;

    for w in inputs {
        let mut x = z;
        x %= 26;
        let mut z = z / divs[depth];
        x += adds[depth];
        x = if x == *w { 0 } else { 1 };
        let mut y = 25;
        y *= x;
        y += 1;
        z *= y;
        y = 0;
        y += w + second_adds[depth];
        y *= x;
        z += y;

        // reached success state
        if z == 0 && depth == 13 {
            return Some(model_number + *w as usize);
        }

        let return_val = run_sub_program(
            inputs,
            bad_states,
            model_number + *w as usize,
            z,
            depth + 1,
            divs,
            adds,
            second_adds,
        );
        if return_val.is_some() {
            return return_val;
        }
    }

    // because of the way z accumulates in the program, we know that at this depth level it simply doesnt work
    // this acts as an operational cache
    bad_states.insert((depth, z));

    None
}

fn main() {
    let divs = vec![1, 1, 1, 26, 26, 1, 1, 1, 26, 26, 26, 1, 26, 26];
    let adds = vec![12, 12, 15, -8, -4, 15, 14, 14, -13, -3, -7, 10, -6, -8];
    let second_adds = vec![1, 1, 16, 5, 9, 3, 2, 15, 5, 11, 7, 1, 10, 3];

    let mut bad_states: BadState = HashSet::new();

    let mn = run_sub_program(
        &(1..=9).rev().collect(),
        &mut bad_states,
        0,
        0,
        0,
        &divs,
        &adds,
        &second_adds,
    );
    println!("{:?}", mn);

    let mn = run_sub_program(
        &(1..=9).collect(),
        &mut bad_states,
        0,
        0,
        0,
        &divs,
        &adds,
        &second_adds,
    );
    println!("{:?}", mn);
}
