use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    let file_path = args().nth(1).expect("No filepath provided");

    let file_reader = read_lines(file_path).expect("Could not read file");

    let mut floor = 0;
    let mut char_counter = 0;
    let mut basement_entered = false;

    for line in file_reader {
        for char in line.unwrap_or_default().chars() {
            char_counter += 1;
            match char {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if !basement_entered && floor == -1 {
                basement_entered = true;
                println!("basement entered at pos {}", char_counter)
            }
        }
    }
    println!("floor {}", floor)
}

fn read_lines<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}
