use std::error::Error;

fn main() {
    let input = include_str!("../input.txt");
    match run_part1(input) {
        Ok(output) => println!("The answer is: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}

type Coords = (i32, i32);

const DIRECTION_DELTAS: [Coords; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut position: Coords = (0, 0);
    let mut direction_idx = 0;

    for step in input.trim().split(", ") {
        let mut chars = step.chars();
        let turn = chars.next().ok_or("Empty turn")?;
        let num: i32 = chars
            .collect::<String>()
            .parse()
            .map_err(|_| "failed to parse number")?;

        direction_idx = if turn == 'R' {
            (direction_idx + 1) % 4
        } else {
            (direction_idx + 3) % 4
        };

        let current_direction = DIRECTION_DELTAS[direction_idx];

        position.0 += current_direction.0 * num;
        position.1 += current_direction.1 * num;
    }

    return Ok(position.0.abs() + position.1.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_run_part1 {
        ($name:ident, $s:expr, $o:expr) => {
            #[test]
            fn $name() {
                match run_part1($s) {
                    Ok(o) => assert_eq!($o, o),
                    Err(e) => panic!("{}", e),
                }
            }
        };
    }

    test_run_part1!(two, "R2, L3", 5);
    test_run_part1!(three, "R2, R2, R2", 2);
    test_run_part1!(four, "R5, L5, R5, R3", 12);
}
