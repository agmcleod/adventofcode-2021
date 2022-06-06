use core::num;
use std::cmp::Ordering;
use std::io::Result;

use read_input::read_text;

fn bin_vec_to_num(vec: &Vec<u32>) -> u32 {
    vec.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, value)| sum + (2u32.pow(i as u32) * *value))
}

fn sum_full_list(binary_values: &Vec<Vec<u32>>, size: usize) -> Vec<u32> {
    binary_values
        .iter()
        .fold(vec![0; size], |mut sum_vec, row| {
            for (i, v) in row.iter().enumerate() {
                sum_vec[i] += *v;
            }

            sum_vec
        })
}

fn calculate_rates(sum_vec: &Vec<u32>, size: usize, num_of_entries: u32) -> (Vec<u32>, Vec<u32>) {
    let mut gamma_rate = vec![0; size];
    let mut epsilon_rate = vec![0; size];

    for (i, v) in sum_vec.iter().enumerate() {
        if *v > num_of_entries / 2 {
            gamma_rate[i] = 1;
            epsilon_rate[i] = 0;
        } else {
            gamma_rate[i] = 0;
            epsilon_rate[i] = 1;
        }
    }

    (gamma_rate, epsilon_rate)
}

fn get_bits_ordering(
    binary_values: &Vec<Vec<u32>>,
    index: usize,
    num_of_entries: usize,
) -> Ordering {
    let sum = binary_values
        .iter()
        .fold(0, |sum, binary| sum + binary[index]);

    (sum * 2).cmp(&(num_of_entries as u32))
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

    let num_of_binary_digits = binary_values[0].len();
    let sum_vec = sum_full_list(&binary_values, num_of_binary_digits);

    let (gamma_rate, epsilon_rate) = calculate_rates(&sum_vec, num_of_binary_digits, count);

    let gamma_num = bin_vec_to_num(&gamma_rate);
    let epsilon_num = bin_vec_to_num(&epsilon_rate);
    let consumption = gamma_num * epsilon_num;

    println!("{}", consumption);

    let mut o2_list = binary_values;
    let mut co2_list = o2_list.clone();

    while o2_list.len() > 1 && co2_list.len() > 1 {
        for i in 0..num_of_binary_digits {
            if o2_list.len() > 1 {
                match get_bits_ordering(&o2_list, i, o2_list.len()) {
                    Ordering::Less => {
                        // if there are fewer 1s, keep the 0s
                        o2_list = o2_list.iter().filter(|bin| bin[i] == 0).cloned().collect();
                    }
                    _ => {
                        // if there are more or equal 1s, keep the 1s
                        o2_list = o2_list.iter().filter(|bin| bin[i] == 1).cloned().collect();
                    }
                }
            }

            if co2_list.len() > 1 {
                match get_bits_ordering(&co2_list, i, co2_list.len()) {
                    Ordering::Less => {
                        // if 1s are less common keep them
                        co2_list = co2_list.iter().filter(|bin| bin[i] == 1).cloned().collect();
                    }
                    _ => {
                        // if 0s are less common or it is even, keep the 0s
                        co2_list = co2_list.iter().filter(|bin| bin[i] == 0).cloned().collect();
                    }
                }
            }
        }
    }

    let o2 = bin_vec_to_num(&o2_list[0]);
    let co2 = bin_vec_to_num(&co2_list[0]);

    println!("{} * {} = {}", o2, co2, o2 * co2);

    Ok(())
}
