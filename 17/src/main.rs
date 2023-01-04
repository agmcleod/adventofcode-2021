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
    velocity.1 < 0 && position.1 < target_area.1
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
    // let target_area = (20, -10, 30, -5);
    let target_area = (248, -85, 285, -56);

    let mut highest_y_reached = None;

    let mut result;

    let mut hit_count = 0;
    let range = 1000;
    for x in -range..range {
        for y in -range..range {
            result = try_launch_velocity(&target_area, (x, y));
            match result.0 {
                TrajectoryResult::HitTarget => {
                    if highest_y_reached.is_none() || result.2 > highest_y_reached.unwrap() {
                        highest_y_reached = Some(result.2);
                    }
                    hit_count += 1;
                }
                _ => {}
            }
        }
    }

    println!("{}, {}", highest_y_reached.unwrap(), hit_count);
}
