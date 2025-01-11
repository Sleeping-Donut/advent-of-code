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
    let mut total_ribbon = 0;
    for line_reader in file {
        let line = line_reader.expect("Oh no, error reading line!");

        // surface area =
        // 2*l*w + 2*w*h + 2*h*l
        let (length, width, height) =
            parse_dimensions(line).expect("Could not parse dimensions from file");

        let surface_area = (2 * length * width) + (2 * width * height) + (2 * height * length);

        let smallest = n_smallest(&[length, width, height], 2)
            .expect("Could not find 2 smallest in dimensions");

        // slack area is the product of the 2 smallest dimensions
        let slack_area: i32 = smallest.iter().product();

        // ribbon wrap = smallest perimiter of any one face
        // ribbon bow = cubic volume of the dimensions
        // ribbon length = wrap + bow
        let ribbon_length = (smallest.iter().sum::<i32>() * 2) + (width * length * height);

        total_paper += surface_area + slack_area;
        total_ribbon += ribbon_length;
    }
    println!("total paper: {}", total_paper);
    println!("total ribbon: {}", total_ribbon);
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

fn n_smallest(lengths: &[i32], n: u32) -> Result<Vec<i32>, ()> {
    let mut smallest_numbers: Vec<i32> = vec![];
    match lengths.iter().min() {
        Some(val) => smallest_numbers.push(*val),
        None => return Err(()),
    }
    for _ in 1..n {
        let smallest = match smallest_numbers.last() {
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
            smallest_numbers.push(**val)
        }
    }
    Ok(smallest_numbers)
}
