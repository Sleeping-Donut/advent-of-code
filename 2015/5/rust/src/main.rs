use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = args().nth(1).expect("Must provide file path");
    let file = File::open(file_path).expect("File must exist");
    let file_reader = BufReader::new(file);

    let nice_total: usize = file_reader
        .lines()
        .map(Result::unwrap_or_default)
        .filter(|line| is_string_nice_part1(line))
        .count();

    println!("nice: {}", nice_total);
}

fn is_string_nice(input: &str) -> bool {
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


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn validate_part1_examples() {
		assert!(is_string_nice("ugknbfddgicrmopn"));
		assert!(is_string_nice("aaa"));

		assert!(!is_string_nice("jchzalrnumimnmhp"));
		assert!(!is_string_nice("haegwjzuvuyypxyu"));
		assert!(!is_string_nice("dvszwmarrgswjxmb"));
	}
}
