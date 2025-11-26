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
    let mut triangles: Vec<[i32; 3]> = Vec::new();

    let rows: Vec<[i32; 3]> = input
        .trim()
        .lines()
        .map(|line| -> Result<[i32; 3], Box<dyn Error>> {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;

            let arr: [i32; 3] = nums.try_into().map_err(|_| "expected 3 numbers per line")?;

            return Ok(arr);
        })
        .collect::<Result<Vec<[i32; 3]>, _>>()?;

    for group in rows.chunks(3) {
        let [a1, b1, c1] = group[0];
        let [a2, b2, c2] = group[1];
        let [a3, b3, c3] = group[2];

        triangles.push([a1, a2, a3]);
        triangles.push([b1, b2, b3]);
        triangles.push([c1, c2, c3]);
    }

    for tri in triangles {
        let mut sides = tri;
        sides.sort();

        if (sides[0] + sides[1]) > sides[2] {
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
