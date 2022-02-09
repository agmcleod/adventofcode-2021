use std::io::Result;

use read_input::read_text;

fn p2_check_window(p2_increases: &mut usize, windows: &[usize; 4], index: usize) {
    let mut previous_index: i32 = index as i32 - 1;
    if previous_index < 0 {
        previous_index = 3;
    }
    if windows[index] > windows[previous_index as usize] {
        *p2_increases += 1;
        // println!(
        //     "increase from {} to {}",
        //     windows[previous_index as usize], windows[index]
        // );
    }
}

fn main() -> Result<()> {
    let text = read_text("1/input.txt")?;

    let mut last_value = None;
    let mut increases = 0;
    let mut p2_increases = 0;
    let mut windows = [0, 0, 0, 0];
    let mut window_index = 0;
    for (i, line) in text.lines().enumerate() {
        let n: usize = line
            .parse()
            .map_err(|_err| {
                panic!("Could not parse number: {}", line);
            })
            .unwrap();
        if last_value.is_some() && last_value.unwrap() < n {
            increases += 1;
        }

        if window_index == 0 {
            windows[0] = n;
            if i > 2 {
                windows[2] += n;
                windows[3] += n;
                // with window 2 done, check it against 1
                p2_check_window(&mut p2_increases, &windows, 2);
            }
        } else if window_index == 1 {
            windows[0] += n;
            windows[1] = n;
            if i > 2 {
                windows[3] += n;
                // with window 3 done, check it against 2
                p2_check_window(&mut p2_increases, &windows, 3);
            }
        } else if window_index == 2 {
            windows[0] += n;
            windows[1] += n;
            windows[2] = n;

            if i > 2 {
                // with window 0 done, check it against 3
                p2_check_window(&mut p2_increases, &windows, 0);
            }
        } else if window_index == 3 {
            windows[1] += n;
            windows[2] += n;
            windows[3] = n;
            // with window 1 done, check it against 0
            p2_check_window(&mut p2_increases, &windows, 1);
        }

        window_index += 1;
        if window_index == 4 {
            window_index = 0;
        }

        last_value = Some(n);
    }

    println!("p1 {}", increases);
    println!("p2 {}", p2_increases);

    Ok(())
}
