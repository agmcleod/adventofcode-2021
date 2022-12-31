type TargetArea = (i32, i32, i32, i32);
type Vec2 = (i32, i32);

fn target_area_contains_x(target_area: &TargetArea, x: i32) -> bool {
    x >= target_area.0 && x <= target_area.2
}

fn target_area_contains_pos(target_area: &TargetArea, position: &Vec2) -> bool {
    target_area_contains_x(target_area, position.0)
        && position.1 >= target_area.1
        && position.1 <= target_area.3
}

fn try_launch_velocity(target_area: &TargetArea, mut velocity: Vec2) -> (bool, Vec2, Vec2) {
    let start_velocity = velocity.clone();

    let mut position = (0, 0);
    let mut hit_target = false;

    loop {
        position.0 += velocity.0;
        position.1 += velocity.1;

        if target_area_contains_pos(&target_area, &position) {
            println!("Hit the target area");
            hit_target = true;
            break;
        }

        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < 0 {
            velocity.0 += 1;
        }

        velocity.1 -= 1;

        if velocity.0 == 0 && !target_area_contains_x(&target_area, position.0) {
            println!("Lost x velocity, not in target area {:?}", position);
            break;
        }

        if velocity.1 < 0 && position.1 < target_area.3 {
            println!("Falling below target area {:?}", position);
            break;
        }
    }

    (hit_target, start_velocity, position)
}

fn main() {
    let target_area = (248, -56, 285, -85);

    println!("{:?}", try_launch_velocity(&target_area, (30, 10)));
}
