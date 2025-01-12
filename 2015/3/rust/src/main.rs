use std::{
    collections::HashSet,
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    // read file
    let file_path = args()
        .nth(1)
        .expect("Need to provide an argument for the input file");

    let file = read_lines(file_path).expect("The input file should be there");

    let mut houses = HashSet::new();
    let mut pos_x: isize = 0;
    let mut pos_y: isize = 0;
    let mut visited: usize = 0;

    for line in file {
        for char in line.unwrap_or_default().chars() {
            match char {
                '<' => pos_x -= 1,
                '>' => pos_x += 1,
                'v' => pos_y -= 1,
                '^' => pos_y += 1,
                _ => (),
            }

            if houses.insert(format!("{}-{}", pos_x, pos_y)) {
                visited += 1;
            }
        }
    }
    println!("visited: {}", visited)
}

fn read_lines<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}
