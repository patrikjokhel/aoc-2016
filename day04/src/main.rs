use std::{collections::HashMap, error::Error, fs};

fn main() {
    let input = fs::read_to_string("./day04/input.txt").expect("missing input.txt");
    match run_part1(&input) {
        Ok(val) => println!("Part1 solution is {}", val),
        Err(e) => eprintln!("Part1 Error {}", e),
    }
    match run_part2(&input) {
        Ok(val) => println!("Part2 solution is {}", val),
        Err(e) => eprintln!("Part2 Error {}", e),
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

        self.letters.iter().for_each(|letter| {
            letter
                .chars()
                .for_each(|ch| *character_map.entry(ch).or_insert(0) += 1);
        });

        let mut counts: Vec<(char, i32)> = character_map.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let checksum: String = counts.iter().take(5).map(|(ch, _)| ch).collect();
        return checksum == self.checksum;
    }

    fn decrypt_name(&self) -> String {
        let name = self
            .letters
            .iter()
            .map(|el| {
                el.chars()
                    .map(|ch| shift_char(&ch, self.sector))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" ");

        return name;
    }
}

fn shift_char(ch: &char, shift: i32) -> char {
    return ((((*ch as u8 - b'a') as i32 + shift) % 26) as u8 + b'a') as char;
}

fn run_part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let rooms: Vec<Room> = input
        .lines()
        .map(|line| {
            return Room::from_string(line.trim());
        })
        .collect::<Result<Vec<Room>, _>>()?;

    return Ok(rooms
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| r.sector)
        .sum());
}

fn run_part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let rooms: Vec<Room> = input
        .lines()
        .map(|line| {
            return Room::from_string(line.trim());
        })
        .collect::<Result<Vec<Room>, _>>()?;

    for room in rooms {
        if !room.is_valid() {
            continue;
        }

        if room.decrypt_name().contains("north") {
            return Ok(room.sector);
        }
    }

    return Err("couldnt find room that would match".into());
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

    #[test]
    fn test_decrypt_name() {
        let room = Room::from_string("qzmt-zixmtkozy-ivhz-343[decoy]").unwrap();
        assert_eq!(room.decrypt_name(), "very encrypted name");
    }
}
