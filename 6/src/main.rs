use std::io::Result;

use read_input::read_text;

fn generate_pop(mut lantern_fish: [usize; 9], times: usize) {
    for _ in 0..times {
        lantern_fish =
            lantern_fish
                .iter()
                .enumerate()
                .fold([0; 9], |mut new_lantern_fish, (i, n)| {
                    if i == 0 {
                        new_lantern_fish[6] += n;
                        new_lantern_fish[8] += n;
                    } else {
                        new_lantern_fish[i - 1] += n;
                    }
                    new_lantern_fish
                });
    }

    println!("{}", lantern_fish.iter().sum::<usize>());
}

fn main() -> Result<()> {
    let text = read_text("6/input.txt")?;

    let lantern_fish = text
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .fold([0; 9], |mut lantern_fish, num| {
            lantern_fish[num % 9] += 1;
            lantern_fish
        });

    generate_pop(lantern_fish.clone(), 80);
    generate_pop(lantern_fish, 256);

    Ok(())
}
