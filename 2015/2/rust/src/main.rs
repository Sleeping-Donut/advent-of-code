use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    // read file
    let file_path = args()
        .nth(1)
        .expect("Need to provide an argument for the input file");

    let file = File::open(file_path).expect("The input file must exist");
    let file_reader = BufReader::new(file);

    let christmas_order = calculate_order_quantity(file_reader.lines());
    println!("total paper: {}", christmas_order.paper);
    println!("total ribbon: {}", christmas_order.ribbon);
}

struct ChristmasOrder {
    paper: usize,
    ribbon: usize,
}

fn calculate_order_quantity<I>(lines: I) -> ChristmasOrder
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut total_paper: usize = 0;
    let mut total_ribbon: usize = 0;
    for line_reader in lines {
        let line = line_reader.expect("Oh no, error reading line!");

        // surface area =
        // 2*l*w + 2*w*h + 2*h*l
        let (length, width, height) =
            parse_dimensions(line).expect("Could not parse dimensions from file");

        let surface_area = (2 * length * width) + (2 * width * height) + (2 * height * length);

        let smallest = n_smallest(&[length, width, height], 2)
            .expect("Could not find 2 smallest in dimensions");

        // slack area is the product of the 2 smallest dimensions
        let slack_area: usize = smallest.iter().product();

        // ribbon wrap = smallest perimiter of any one face
        // ribbon bow = cubic volume of the dimensions
        // ribbon length = wrap + bow
        let ribbon_length = (smallest.iter().sum::<usize>() * 2) + (width * length * height);

        total_paper += surface_area + slack_area;
        total_ribbon += ribbon_length;
    }
    ChristmasOrder {
        paper: total_paper,
        ribbon: total_ribbon,
    }
}

fn parse_dimensions(text_dimensions: String) -> Result<(usize, usize, usize), ()> {
    let mut dimensions = text_dimensions.split("x");
    let length = dimensions
        .next()
        .expect("Could not find first dimension")
        .trim()
        .parse::<usize>()
        .expect("Could not parse first dimension");
    let width = dimensions
        .next()
        .expect("Could not find second dimension")
        .trim()
        .parse::<usize>()
        .expect("Could not parse second dimension");
    let height = dimensions
        .next()
        .expect("Could not find second dimension")
        .trim()
        .parse::<usize>()
        .expect("Could not parse second dimension");
    Ok((length, width, height))
}

fn n_smallest(lengths: &[usize], n: u32) -> Result<Vec<usize>, ()> {
    let mut smallest_numbers: Vec<usize> = vec![];
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
        let slice: Vec<&usize> = lengths[..position]
            .iter()
            .chain(lengths[position + 1..].iter())
            .collect();
        if let Some(val) = slice.iter().min() {
            smallest_numbers.push(**val)
        }
    }
    Ok(smallest_numbers)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn validate_part1_examples() {
        let input = "2x3x4";
        let input_reader = Cursor::new(input);
        let christmas_order = calculate_order_quantity(input_reader.lines());
        assert_eq!(christmas_order.paper, 58);

        let input = "1x1x10";
        let input_reader = Cursor::new(input);
        let christmas_order = calculate_order_quantity(input_reader.lines());
        assert_eq!(christmas_order.paper, 43);
    }

    #[test]
    fn validate_part2_examples() {
        let input = "2x3x4";
        let input_reader = Cursor::new(input);
        let christmas_order = calculate_order_quantity(input_reader.lines());
        assert_eq!(christmas_order.ribbon, 34);

        let input = "1x1x10";
        let input_reader = Cursor::new(input);
        let christmas_order = calculate_order_quantity(input_reader.lines());
        assert_eq!(christmas_order.ribbon, 14);
    }
}
