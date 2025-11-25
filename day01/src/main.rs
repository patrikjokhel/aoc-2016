use std::{collections::HashSet, error::Error};

fn main() {
    let input = include_str!("../input.txt");
    match run_part1(input) {
        Ok(output) => println!("The first part answer is: {}", output),
        Err(e) => eprintln!("Part1 Error: {}", e),
    }
    match run_part2(input) {
        Ok(output) => println!("The second part answer is: {}", output),
        Err(e) => eprintln!("Part2 Error: {}", e),
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

fn run_part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut position: Coords = (0, 0);
    let mut direction_idx = 0;
    let mut positions_visited = HashSet::new();

    'outer: for step in input.trim().split(", ") {
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

        if current_direction.0 != 0 {
            for _ in 0..num {
                position.0 += current_direction.0;
                let newly_inserted = positions_visited.insert(position);
                if !newly_inserted {
                    break 'outer;
                }
            }
        } else {
            for _ in 0..num {
                position.1 += current_direction.1;
                let newly_inserted = positions_visited.insert(position);
                if !newly_inserted {
                    break 'outer;
                }
            }
        }
    }

    return Ok(position.0.abs() + position.1.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_run {
        ($name:ident, $func:expr, $s:expr, $o:expr) => {
            #[test]
            fn $name() {
                match $func($s) {
                    Ok(o) => assert_eq!($o, o),
                    Err(e) => panic!("{}", e),
                }
            }
        };
    }

    test_run!(example1, run_part1, "R2, L3", 5);
    test_run!(example2, run_part1, "R2, R2, R2", 2);
    test_run!(example3, run_part1, "R5, L5, R5, R3", 12);
    test_run!(exampl4, run_part2, "R8, R4, R4, R8", 4);
}
