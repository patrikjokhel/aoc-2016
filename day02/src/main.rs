use std::{
    cmp::{max, min},
    collections::HashMap,
    error::Error,
    fs,
};

fn main() {
    let input = fs::read_to_string("./day02/input.txt").expect("input file is missing");
    match run_part1(&input) {
        Ok(res) => println!("The part1 solution is: {}", res),
        Err(e) => eprintln!("Part1 Error {}", e),
    }
    match run_part2(&input) {
        Ok(res) => println!("The part2 solution is: {}", res),
        Err(e) => eprintln!("Part2 Error {}", e),
    }
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let numbers: HashMap<(i32, i32), i32> = HashMap::from([
        ((-1, 1), 1),
        ((0, 1), 2),
        ((1, 1), 3),
        ((-1, 0), 4),
        ((0, 0), 5),
        ((1, 0), 6),
        ((-1, -1), 7),
        ((0, -1), 8),
        ((1, -1), 9),
    ]);

    let mut position: (i32, i32) = (0, 0);
    let mut num_str = String::new();

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => position.1 = min(position.1 + 1, 1),
                'D' => position.1 = max(position.1 - 1, -1),
                'L' => position.0 = max(position.0 - 1, -1),
                'R' => position.0 = min(position.0 + 1, 1),
                _ => return Err("invalid character received".into()),
            }
        }

        match numbers.get(&position) {
            Some(n) => num_str += n.to_string().as_str(),
            None => return Err(format!("position is not in numbers: {:?}", position).into()),
        }
    }

    let result: i32 = num_str.parse()?;
    return Ok(result);
}

fn run_part2(input: &str) -> Result<String, Box<dyn Error>> {
    let numbers: HashMap<(i32, i32), &str> = HashMap::from([
        ((0, 2), "1"),
        ((-1, 1), "2"),
        ((0, 1), "3"),
        ((1, 1), "4"),
        ((-2, 0), "5"),
        ((-1, 0), "6"),
        ((0, 0), "7"),
        ((1, 0), "8"),
        ((2, 0), "9"),
        ((-1, -1), "A"),
        ((0, -1), "B"),
        ((1, -1), "C"),
        ((0, -2), "D"),
    ]);

    let mut position: (i32, i32) = (-2, 0);
    let mut num_str = String::new();

    for line in input.lines() {
        for c in line.chars() {
            let mut new_position = position.clone();
            match c {
                'U' => new_position.1 = new_position.1 + 1,
                'D' => new_position.1 = new_position.1 - 1,
                'L' => new_position.0 = new_position.0 - 1,
                'R' => new_position.0 = new_position.0 + 1,
                _ => return Err("invalid character received".into()),
            }

            if numbers.contains_key(&new_position) {
                position = new_position
            }
        }

        match numbers.get(&position) {
            Some(n) => num_str += n,
            None => return Err(format!("position is not in numbers: {:?}", position).into()),
        }
    }

    return Ok(num_str.to_string());
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
                    Err(e) => panic!("Error {}", e),
                }
            }
        };
    }

    test_run!(example_1, run_part1, "ULL", 1);
    test_run!(
        example_2,
        run_part1,
        "ULL
RRDDD
LURDL
UUUUD",
        1985
    );
    test_run!(example_3, run_part2, "ULL", "5");
    test_run!(
        example_4,
        run_part2,
        "ULL
RRDDD
LURDL
UUUUD",
        "5DB3"
    );
}
