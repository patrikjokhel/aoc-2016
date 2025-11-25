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
                _ => panic!("invalid character received: {}", c),
            }
        }

        match numbers.get(&position) {
            Some(n) => num_str += n.to_string().as_str(),
            None => panic!("position is not in numbers: {:?}", position),
        }
    }

    let result: i32 = match num_str.parse() {
        Ok(n) => n,
        Err(_) => panic!("result is not a valid i32: {}", num_str),
    };

    return Ok(result);
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
}
