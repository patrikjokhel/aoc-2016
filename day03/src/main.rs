use std::{error::Error, fs};

fn main() {
    let input = fs::read_to_string("./day03/input.txt").expect("missing input.txt");
    match run_part1(&input) {
        Ok(res) => println!("Part1 solution is: {}", res),
        Err(e) => eprintln!("Part1 Error {}", e),
    }
    match run_part2(&input) {
        Ok(res) => println!("Part2 solution is: {}", res),
        Err(e) => eprintln!("Part2 Error {}", e),
    }
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut valid_triangle_count: i32 = 0;

    for line in input.trim().lines() {
        let mut nums: [i32; 3] = line
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?
            .try_into()
            .map_err(|_| "expected exactly 3 numbers per row")?;

        nums.sort();

        if (nums[0] + nums[1]) > nums[2] {
            valid_triangle_count += 1;
        }
    }
    return Ok(valid_triangle_count);
}

fn run_part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut valid_triangle_count: i32 = 0;

    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    let mut c: Vec<i32> = Vec::new();

    let nums = input.trim().lines().map(|line| {
        line.split("  ")
            .filter_map(|n| match n.trim().parse::<i32>() {
                Ok(val) => Some(val),
                _ => None,
            })
    });

    for line in nums {
        let cols: Vec<i32> = line.collect();
        if cols.len() != 3 {
            return Err("row without 3 cols encountered".into());
        }
        a.push(cols[0]);
        b.push(cols[1]);
        c.push(cols[2]);
    }

    a.append(&mut b);
    a.append(&mut c);

    for chunk in a.chunks_mut(3) {
        if chunk.len() != 3 {
            return Err("triangle without 3 sides encountered".into());
        }
        chunk.sort();

        if (chunk[0] + chunk[1]) > chunk[2] {
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
    run_test!(
        example_3,
        run_part2,
        "  5  10  25
  10  15  20
  25  20  10",
        2
    );
}
