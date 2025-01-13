use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let file_path = args().nth(1).expect("No filepath provided");

    let file = File::open(file_path).expect("File must exist");
    let file_reader = BufReader::new(file);

    let (floor, basement_position) = santa_floors(file_reader.lines());

    println!("floor {}", floor);
    if let Some(position) = basement_position {
        println!("basement entered at pos {}", position)
    }
}

fn santa_floors<I>(lines: I) -> (isize, Option<usize>)
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut floor: isize = 0;
    let mut char_counter = 0;
    let mut basement_entered = None;

    for line in lines {
        for char in line.unwrap_or_default().chars() {
            char_counter += 1;
            match char {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if basement_entered.is_none() && floor == -1 {
                basement_entered = Some(char_counter);
            }
        }
    }
    (floor, basement_entered)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn validate_part1_examples() {
        validate_part1("(())", 0);
        validate_part1("()()", 0);
        validate_part1("(((", 3);
        validate_part1("(()(()(", 3);
        validate_part1("))(((((", 3);
        validate_part1("())", -1);
        validate_part1("))(", -1);
        validate_part1(")))", -3);
        validate_part1(")())())", -3);
    }

    fn validate_part1(input: &str, result: isize) {
        let input_reader = Cursor::new(input);
        let (floor, _) = santa_floors(input_reader.lines());
        assert_eq!(floor, result);
    }

    #[test]
    fn validate_part2_examples() {
        validate_part2(")", 1);
        validate_part2("()())", 5);
    }

    fn validate_part2(input: &str, result: usize) {
        let input_reader = Cursor::new(input);
        let (_, basement_position) = santa_floors(input_reader.lines());
        match basement_position {
            Some(position) => assert_eq!(position, result),
            None => panic!("failed"),
        }
    }
}
