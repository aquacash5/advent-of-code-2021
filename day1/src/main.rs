use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day1", about = "Sonar Sweep")]
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
        let data: Vec<u32> = lines
            .filter_map(Result::ok)
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let part_1 = data.windows(2).filter(|a| a.first() < a.last()).count();
        let part_2 = data.windows(4).filter(|a| a.first() < a.last()).count();
        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
