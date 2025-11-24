fn main() {
    let input = include_str!("../input.txt");
    println!("The answer is: {}", run(input.trim()));
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn run(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut current_direction = Direction::North;

    for step in input.split(", ") {
        let dir = &step[..1];
        let num: i32 = step[1..].parse().unwrap();

        match current_direction {
            Direction::North => {
                if dir == "R" {
                    position.0 += num;
                    current_direction = Direction::East;
                } else {
                    position.0 -= num;
                    current_direction = Direction::West;
                }
            }
            Direction::East => {
                if dir == "R" {
                    position.1 -= num;
                    current_direction = Direction::South;
                } else {
                    position.1 += num;
                    current_direction = Direction::North;
                }
            }
            Direction::South => {
                if dir == "R" {
                    position.0 -= num;
                    current_direction = Direction::West;
                } else {
                    position.0 += num;
                    current_direction = Direction::East;
                }
            }
            Direction::West => {
                if dir == "R" {
                    position.1 += num;
                    current_direction = Direction::North;
                } else {
                    position.1 -= num;
                    current_direction = Direction::South;
                }
            }
        }
    }
    return position.0.abs() + position.1.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_run {
        ($name:ident, $s:expr, $o:expr) => {
            #[test]
            fn $name() {
                assert_eq!($o, run($s))
            }
        };
    }

    test_run!(two, "R2, L3", 5);
    test_run!(three, "R2, R2, R2", 2);
    test_run!(four, "R5, L5, R5, R3", 12);
}
