fn decrement_number(number_digits: &mut Vec<i32>) {
    if number_digits.len() != 14 {
        panic!(
            "Invalid length of digits, should be 14. was {}",
            number_digits.len()
        );
    }
    let mut index_to_decrement = 13;
    loop {
        let digit_value = number_digits.get_mut(index_to_decrement as usize).unwrap();
        *digit_value -= 1;
        if *digit_value == 0 {
            *digit_value = 9;
            index_to_decrement -= 1;
            if index_to_decrement < 0 {
                break;
            }
        } else {
            break;
        }
    }
}

fn main() {
    let mut inputs = vec![9; 14];

    let divs = vec![1, 1, 1, 26, 26, 1, 1, 1, 26, 26, 26, 1, 26, 26];
    let adds = vec![12, 12, 15, -8, -4, 15, 14, 14, -13, -3, -7, 10, -6, -8];
    let second_adds = vec![1, 1, 16, 5, 9, 3, 2, 15, 5, 11, 7, 1, 10, 3];

    loop {
        let mut x;
        let mut y;
        let mut z = 0;

        for (i, inp) in inputs.iter().enumerate() {
            let w = *inp;

            x = z;
            x %= 26;
            z /= divs[i];
            x += adds[i];
            x = if x == w { 0 } else { 1 };
            y = 25;
            y *= x;
            y += 1;
            z *= y;
            y = 0;
            y += w + second_adds[i];
            y *= x;
            z += y;
        }

        if z == 0 {
            println!(
                "{}",
                inputs.iter().map(|n| n.to_string()).collect::<String>()
            );
            break;
        } else {
            decrement_number(&mut inputs);
        }
    }
}
