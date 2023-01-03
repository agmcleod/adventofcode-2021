type TargetArea = (i32, i32, i32, i32);
type Vec2 = (i32, i32);

#[derive(Debug)]
enum TrajectoryResult {
    HitTarget,
    ShortOfTarget,
    PassedTarget,
    FallingShortOfTarget,
    FallingPassedTarget,
    FellOverTarget,
}

fn target_area_contains_x(target_area: &TargetArea, x: i32) -> bool {
    x >= target_area.0 && x <= target_area.2
}

fn target_area_contains_pos(target_area: &TargetArea, position: &Vec2) -> bool {
    target_area_contains_x(target_area, position.0)
        && position.1 >= target_area.1
        && position.1 <= target_area.3
}

fn falling_below_target_area(target_area: &TargetArea, position: &Vec2, velocity: &Vec2) -> bool {
    velocity.1 < 0 && position.1 < target_area.3
}

fn halve_velocity(velocity_value: &mut i32) {
    if *velocity_value != 1 && *velocity_value != -1 {
        *velocity_value /= 2;
    }
}

fn try_launch_velocity(
    target_area: &TargetArea,
    mut velocity: Vec2,
) -> (TrajectoryResult, Vec2, i32) {
    let mut position = (0, 0);
    let mut highest_y_point = std::i32::MIN;

    let result;

    loop {
        position.0 += velocity.0;
        position.1 += velocity.1;

        highest_y_point = highest_y_point.max(position.1);

        if target_area_contains_pos(&target_area, &position) {
            println!("Hit the target area");
            result = TrajectoryResult::HitTarget;
            break;
        }

        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < 0 {
            velocity.0 += 1;
        }

        velocity.1 -= 1;
        // left of target
        if position.0 < target_area.0 {
            if velocity.0 <= 0 {
                result = TrajectoryResult::ShortOfTarget;
                break;
            }

            if falling_below_target_area(target_area, &position, &velocity) {
                result = TrajectoryResult::FallingShortOfTarget;
                break;
            }
        }

        // right of target
        if position.0 > target_area.2 {
            if velocity.0 >= 0 {
                result = TrajectoryResult::PassedTarget;
                break;
            }

            if falling_below_target_area(target_area, &position, &velocity) {
                result = TrajectoryResult::FallingPassedTarget;
                break;
            }
        }

        if falling_below_target_area(&target_area, &position, &velocity)
            && target_area_contains_x(&target_area, position.0)
        {
            result = TrajectoryResult::FellOverTarget;
            break;
        }
    }

    (result, position, highest_y_point)
}

fn main() {
    let target_area = (20, -5, 30, -10);
    // let target_area = (248, -56, 285, -85);

    let mut velocity = (2, 2);
    let mut highest_y_reached = None;
    let mut increase_rate_x = 3;
    let mut increase_rate_y = 3;

    loop {
        let result = try_launch_velocity(&target_area, velocity.clone());
        println!("{:?} {:?}", result, velocity);

        match result.0 {
            TrajectoryResult::HitTarget => {
                if highest_y_reached.is_none() || result.2 > highest_y_reached.unwrap() {
                    highest_y_reached = Some(result.2);
                    velocity.1 += 1;
                    velocity.0 += 1;
                } else {
                    // y point is decreasing, so let's assume highest was reached
                    break;
                }
            }
            TrajectoryResult::ShortOfTarget => {
                velocity.0 += increase_rate_x;
            }
            TrajectoryResult::PassedTarget => {
                if highest_y_reached.is_some() {
                    break;
                }
                halve_velocity(&mut increase_rate_x);
                velocity.0 -= increase_rate_x;
            }
            TrajectoryResult::FallingShortOfTarget => {
                velocity.0 += increase_rate_x;
                velocity.1 += increase_rate_y;
            }
            TrajectoryResult::FallingPassedTarget => {
                if highest_y_reached.is_some() {
                    break;
                }
                // reduce x rate as it means that we're falling after passing the area, so we can reduce x velocity to try to land closer to the area.
                halve_velocity(&mut increase_rate_x);
                velocity.0 -= increase_rate_x;
            }
            TrajectoryResult::FellOverTarget => {
                if highest_y_reached.is_some() {
                    break;
                }
                halve_velocity(&mut increase_rate_y);
                velocity.1 -= increase_rate_y;
            }
        }
    }

    println!("{:?}", velocity);
}
