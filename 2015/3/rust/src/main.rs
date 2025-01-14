use std::{
    collections::HashSet,
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
};

fn main() {
    // read file
    let file_path = args()
        .nth(1)
        .expect("Need to provide an argument for the input file");

    let file = File::open(file_path).expect("File must exist");
    let mut file_reader = BufReader::new(file);

    let single_santa_visited = santas_visits(file_reader.by_ref().lines(), 1);
    println!("visited: {}", single_santa_visited);

    file_reader
        .get_mut()
        .seek(io::SeekFrom::Start(0))
        .expect("Cannot seek to beginning");
    let with_robo_visits = santas_visits(file_reader.by_ref().lines(), 2);
    println!("visited: {}", with_robo_visits);
}

#[derive(Default, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

fn santas_visits<I>(lines: I, n: usize) -> usize
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut houses: HashSet<Position> = HashSet::new();
    let mut santas_pos: Vec<Position> = vec![Position::default(); n];
    let mut visited: usize = 0;
    let mut which_santa = 0;

    // Starting location counts as visited
    houses.insert(Position { x: 0, y: 0 }); // starting pos
    visited += 1;

    for line in lines {
        for char in line.unwrap_or_default().chars() {
            let (xd, yd) = match char {
                '<' => (-1, 0),
                '>' => (1, 0),
                'v' => (0, -1),
                '^' => (0, 1),
                _ => (0, 0),
            };

            santas_pos[which_santa].x += xd;
            santas_pos[which_santa].y += yd;
            if houses.insert(Position {
                x: santas_pos[which_santa].x,
                y: santas_pos[which_santa].y,
            }) {
                visited += 1;
            }
            which_santa = (which_santa + 1) % n;
        }
    }
    visited
}

#[cfg(test)]
mod test {
    use io::Cursor;

    use super::*;

    #[test]
    fn verify_task1_examples() {
        let input_reader = Cursor::new(">").lines();
        let visited = santas_visits(input_reader, 1);
        assert_eq!(visited, 2);

        let input_reader = Cursor::new("^>v<").lines();
        let visited = santas_visits(input_reader, 1);
        assert_eq!(visited, 4);

        let input_reader = Cursor::new("^v^v^v^v^v").lines();
        let visited = santas_visits(input_reader, 1);
        assert_eq!(visited, 2);
    }

    #[test]
    fn verify_task2_examples() {
        let input_reader = Cursor::new("^v").lines();
        let visited = santas_visits(input_reader, 2);
        assert_eq!(visited, 3);

        let input_reader = Cursor::new("^>v<").lines();
        let visited = santas_visits(input_reader, 2);
        assert_eq!(visited, 3);

        let input_reader = Cursor::new("^v^v^v^v^v").lines();
        let visited = santas_visits(input_reader, 2);
        assert_eq!(visited, 11);
    }
}
