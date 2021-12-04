use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
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
        let data: i32 = lines
            .filter_map(Result::ok)
            .filter_map(|s| s.parse::<i32>().ok())
            // Enable to solve part 2
            // .tuple_windows::<(_, _, _)>()
            // .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum();
        println!("{:?}", data);
    }
}
