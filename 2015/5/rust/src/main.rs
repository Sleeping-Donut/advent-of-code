use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
};

fn main() {
    let file_path = args().nth(1).expect("Must provide file path");
    let file = File::open(file_path).expect("File must exist");
    let mut file_reader = BufReader::new(file);

    let nice_total_part1: usize = file_reader
        .by_ref()
        .lines()
        .map(Result::unwrap_or_default)
        .filter(|line| is_string_nice_part1(line))
        .count();

    file_reader
        .get_mut()
        .seek(io::SeekFrom::Start(0))
        .expect("Cannot seek to start of file");
    let nice_total_part2: usize = file_reader
        .by_ref()
        .lines()
        .map(Result::unwrap_or_default)
        .filter(|line| is_string_nice_part2(line))
        .count();

    println!("nice pt 1: {}", nice_total_part1);
    println!("nice pt 2: {}", nice_total_part2);
}

fn is_string_nice_part1(input: &str) -> bool {
    if input.len() < 3 {
        return false;
    }
    let has_min_3_vowels = 3
        <= input
            .chars()
            .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
            .count();
    let contains_double_char = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(c1, c2)| c1 == c2);
    let contains_banned_strings = ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|pattern| input.contains(pattern));
    has_min_3_vowels && contains_double_char && !contains_banned_strings
}

fn is_string_nice_part2(input: &str) -> bool {
    if input.len() < 4 {
        return false;
    }
    let has_double_pair = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(a, b)| input.split(format!("{}{}", a, b).as_str()).count() > 2);
    	// ^ need to use format instead of &[a,b] as latter for pattern uses OR op
    let has_aba_repeat = input
        .chars()
        .zip(input.chars().skip(1).zip(input.chars().skip(2))) // get tuple of 3 chars
        .map(|(a, (b, c))| (a, b, c)) // flatten tuple
        .any(|(a, _b, c)| a == c);
    has_double_pair && has_aba_repeat
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_part1_examples() {
        assert!(is_string_nice_part1("ugknbfddgicrmopn"));
        assert!(is_string_nice_part1("aaa"));

        assert!(!is_string_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_string_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_string_nice_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn validate_part2_examples() {
        assert!(is_string_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_string_nice_part2("xxyxx"));

        assert!(!is_string_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_string_nice_part2("ieodomkazucvgmuy"));
    }
}
