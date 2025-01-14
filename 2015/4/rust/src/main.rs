use std::{env::args, fs::read_to_string};

use md5::{Digest, Md5};

fn main() {
    let file_path = args().nth(1).expect("Must provide input file");
    let secret_key = read_to_string(file_path).expect("Could not parse file");

    let found_number_5 = find_min_num_for_md5(&secret_key, 5);
    match found_number_5 {
        Some(num) => println!("number for key where 5 0s: {}", num),
        None => println!("It's too long man..."),
    }

    let found_number_6 = find_min_num_for_md5(&secret_key, 6);
    match found_number_6 {
        Some(num) => println!("number for key and 6 0s: {}", num),
        None => println!("It's too long man..."),
    }
}

fn find_min_num_for_md5(secret_key: &String, n: usize) -> Option<usize> {
    let mut found_number: Option<usize> = None;
    let mut count: usize = 0;

    while found_number.is_none() || count >= 100_000_000 {
        let hash = md5_hash(format!("{}{}", secret_key, count));
        if hash[..n].chars().all(|c| c == '0') {
            found_number = Some(count);
            break;
        }
        count += 1;
    }
    found_number
}

fn md5_hash(input: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_part1_examples() {
        let secret_key = "abcdef".to_string();
        let found_number = find_min_num_for_md5(&secret_key, 5).expect("Did not find number");
        assert_eq!(found_number, 609043);

        let secret_key = "pqrstuv".to_string();
        let found_number = find_min_num_for_md5(&secret_key, 5).expect("Did not find number");
        assert_eq!(found_number, 1048970);
    }

    // #[test]
    // fn validate_part2_examples() {}
}
