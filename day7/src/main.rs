use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day7", about = "The Treachery of Whales")]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

trait AbsoluteDifference {
    fn diff_abs(self, other: Self) -> Self;
}

impl AbsoluteDifference for u32 {
    fn diff_abs(self, other: Self) -> Self {
        if self < other {
            other - self
        } else {
            self - other
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_fuel_cost_p1(xs: &Vec<u32>, pos: u32) -> u32 {
    xs.iter().map(|i| i.diff_abs(pos)).sum()
}

fn calc_fuel_cost_p2(xs: &Vec<u32>, pos: u32) -> u32 {
    xs.iter().map(|i| (1..=i.diff_abs(pos)).sum::<u32>()).sum()
}

fn main() {
    let cli = Cli::from_args();
    if let Ok(lines) = read_lines(&cli.input) {
        let data: Vec<u32> = lines
            .filter_map(Result::ok)
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let part_1: u32 = (0..*data.iter().max().unwrap())
            .map(|i| calc_fuel_cost_p1(&data, i))
            .min()
            .unwrap();
        let part_2: u32 = (0..*data.iter().max().unwrap())
            .map(|i| calc_fuel_cost_p2(&data, i))
            .min()
            .unwrap();
        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);
    }
}
