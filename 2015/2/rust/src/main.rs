use std::{
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

    let mut total_paper = 0;
    for line_reader in file {
        let line = line_reader.expect("Oh no, error reading line!");
        // surface area =
        // 2*l*w + 2*w*h + 2*h*l
        let (length, width, height) =
            parse_dimensions(line).expect("Could not parse dimensions from file");
        let surface_area = (2 * length * width) + (2 * width * height) + (2 * height * length);

        // slack area is the product of the 2 smallest dimensions
        let slack_area = product_of_n_smallest(vec![length, width, height], 2)
            .expect("n smallest larger than vec size");

        total_paper += surface_area + slack_area;
    }
    println!("total paper: {}", total_paper)
}

fn read_lines<P>(file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn parse_dimensions(text_dimensions: String) -> Result<(i32, i32, i32), ()> {
    let mut dimensions = text_dimensions.split("x");
    let length = dimensions
        .next()
        .expect("Could not find first dimension")
        .trim()
        .parse::<i32>()
        .expect("Could not parse first dimension");
    let width = dimensions
        .next()
        .expect("Could not find second dimension")
        .trim()
        .parse::<i32>()
        .expect("Could not parse second dimension");
    let height = dimensions
        .next()
        .expect("Could not find second dimension")
        .trim()
        .parse::<i32>()
        .expect("Could not parse second dimension");
    Ok((length, width, height))
}

fn product_of_n_smallest(lengths: Vec<i32>, n: u32) -> Result<i32, ()> {
    let mut n_smallest: Vec<i32> = vec![];
    match lengths.iter().min() {
        Some(val) => n_smallest.push(*val),
        None => return Err(()),
    }
    for _ in 1..n {
        let smallest = match n_smallest.last() {
            Some(val) => val,
            None => return Err(()),
        };
        let position = lengths
            .iter()
            .position(|val| val == smallest)
            .expect("Value should exist as it was just found");
        let slice: Vec<&i32> = lengths[..position]
            .iter()
            .chain(lengths[position + 1..].iter())
            .collect();
        if let Some(val) = slice.iter().min() {
            n_smallest.push(**val)
        }
    }
    Ok(n_smallest
        .iter()
        .copied()
        .reduce(|acc, val| acc * val)
        .expect("There should be at least one element in n_smallest as long as n > 0"))
}
