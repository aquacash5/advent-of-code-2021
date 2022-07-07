use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day6", about = "Lanternfish")]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
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

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let fishes: Vec<usize> = lines
            .filter_map(Result::ok)
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut gens = [0u64; 9];
        for fish in fishes {
            gens[fish] += 1;
        }
        let mut part_1 = 0u64;
        for i in 0..256 {
            if i == 80 {
                part_1 = gens.iter().sum()
            }
            // let [a, b, c, d, e, f, g, h, i] = gens;
            // gens = [b, c, d, e, f, g, h + a, i, a];
            gens.rotate_left(1);
            gens[6] += gens[8];
        }
        let part_2: u64 = gens.iter().sum();
        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);
    }
}
