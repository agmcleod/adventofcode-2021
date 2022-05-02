use std::io::Result;

use read_input::read_text;

fn bin_vec_to_num(vec: &Vec<u32>) -> u32 {
    vec.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, value)| sum + (2u32.pow(i as u32) * *value))
}

fn main() -> Result<()> {
    let text = read_text("3/input.txt")?;

    let count = text.lines().count();

    let mut binary_values: Vec<Vec<u32>> = Vec::with_capacity(count);
    let count = count as u32;
    for line in text.lines() {
        let binary = line.chars().map(|v| v.to_digit(10).unwrap()).collect();
        binary_values.push(binary);
    }

    let binary_digits = binary_values[0].len();
    let sum_vec = binary_values
        .iter()
        .fold(vec![0; binary_digits], |mut sum_vec, row| {
            for (i, v) in row.iter().enumerate() {
                sum_vec[i] += *v;
            }

            sum_vec
        });

    let mut gamma_rate = vec![0; binary_digits];
    let mut epsilon_rate = vec![0; binary_digits];

    for (i, v) in sum_vec.iter().enumerate() {
        if *v > count / 2 {
            gamma_rate[i] = 1;
            epsilon_rate[i] = 0;
        } else {
            gamma_rate[i] = 0;
            epsilon_rate[i] = 1;
        }
    }

    let gamma_num = bin_vec_to_num(&gamma_rate);
    let epsilon_num = bin_vec_to_num(&epsilon_rate);
    let consumption = gamma_num * epsilon_num;

    println!("{}", consumption);

    Ok(())
}
