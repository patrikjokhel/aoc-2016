use std::{cmp::Ordering, collections::HashMap, error::Error, fs, ops::Add};

fn main() {
    let input = fs::read_to_string("./day04/input.txt").expect("missing input.txt");
    match run_part1(&input) {
        Ok(val) => println!("Part1 solution is {}", val),
        Err(e) => eprintln!("Part1 Error {}", e),
    }
}

struct Room {
    letters: Vec<String>,
    sector: i32,
    checksum: String,
}

impl Room {
    fn from_string(s: &str) -> Result<Room, Box<dyn Error>> {
        let parts: Vec<&str> = s.split("-").collect();

        let letters: Vec<&str> = parts[..parts.len() - 1].to_vec();
        let ident_parts: [&str; 2] = parts[parts.len() - 1]
            .split("[")
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| "expected sector and checksum")?;
        let sector: i32 = ident_parts[0].parse()?;
        let checksum: &str = &ident_parts[1][..ident_parts[1].len() - 1];

        Ok(Room {
            letters: letters.iter().map(|s| s.to_string()).collect(),
            sector,
            checksum: checksum.to_string(),
        })
    }

    fn is_valid(&self) -> bool {
        let mut character_map: HashMap<char, i32> = HashMap::new();
        for letter in &self.letters {
            for ch in letter.chars() {
                match character_map.get_mut(&ch) {
                    Some(val) => {
                        let new_val = val.add(1);
                        character_map.insert(ch, new_val);
                    }
                    None => {
                        character_map.insert(ch, 1);
                    }
                }
            }
        }

        let mut checksum: Vec<(&char, &i32)> = character_map.iter().collect();

        checksum.sort_by(|a, b| {
            if a.1 > b.1 {
                return Ordering::Less;
            } else if a.1 < b.1 {
                return Ordering::Greater;
            }

            return a.0.cmp(b.0);
        });

        let checksum: String = checksum
            .iter()
            .fold(String::from(""), |mut acc, el| {
                acc += &el.0.to_string();
                return acc;
            })
            .chars()
            .take(5)
            .collect();

        return checksum.eq(&self.checksum);
    }
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let rooms: Vec<Room> = input
        .lines()
        .map(|line| {
            return Room::from_string(line.trim());
        })
        .collect::<Result<Vec<Room>, _>>()?
        .try_into()
        .map_err(|_| "boo")?;

    let total: i32 = rooms.iter().fold(0, |mut acc, room| {
        if room.is_valid() {
            acc += room.sector;
        }

        return acc;
    });

    return Ok(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! run_test {
        ($name: ident, $func: expr, $s: expr, $o: expr) => {
            #[test]
            fn $name() {
                match $func($s) {
                    Ok(o) => assert_eq!(o, $o),
                    Err(e) => panic!("Error {}", e),
                }
            }
        };
    }

    run_test!(
        example_1,
        run_part1,
        "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
        1514
    );
}
