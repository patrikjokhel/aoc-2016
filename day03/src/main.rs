use std::{error::Error, fs};

fn main() {
    let input = fs::read_to_string("./day03/input.txt").expect("missing input.txt");
    match run_part1(&input) {
        Ok(res) => println!("Part1 solution is: {}", res),
        Err(e) => eprintln!("Part1 Error {}", e),
    }
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut valid_triangle_count: i32 = 0;

    for (i, line) in input.trim().lines().enumerate() {
        let mut nums: Vec<i32> = line
            .split("  ")
            .filter_map(|n| match n.trim().parse::<i32>() {
                Ok(val) => Some(val),
                _ => None,
            })
            .collect();

        if nums.len() != 3 {
            return Err(format!("triangle without 3 sides {:?} on line {}", nums, i).into());
        }
        nums.sort();

        if (nums[0] + nums[1]) > nums[2] {
            valid_triangle_count += 1;
        }
    }
    return Ok(valid_triangle_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! run_test {
        ($name:ident, $func:expr, $s: expr, $o:expr) => {
            #[test]
            fn $name() {
                match $func($s) {
                    Ok(o) => assert_eq!($o, o),
                    Err(e) => panic!("Error {}", e),
                }
            }
        };
    }

    run_test!(example_1, run_part1, "  5  10  25", 0);
    run_test!(
        example_2,
        run_part1,
        "  5  10  25
  10  15  20
  5  7  11
  5  5  15",
        2
    );
}
