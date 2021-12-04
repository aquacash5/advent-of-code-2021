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
        let data: Vec<_> = lines.filter_map(Result::ok).collect();
        let mut binary = vec![vec!['0'; 0]; data[0].len()];
        for row in data {
            for (i, col) in row.chars().enumerate() {
                binary[i].push(col)
            }
        }
        let (gamma, epsilon) = binary
            .into_iter()
            .map(|c| c.into_iter().counts())
            .map(|c| {
                if c[&'0'] > c[&'1'] {
                    (String::from("0"), String::from("1"))
                } else {
                    (String::from("1"), String::from("0"))
                }
            })
            .fold((String::from(""), String::from("")), |v, (g, e)| {
                (format!("{}{}", v.0, g), format!("{}{}", v.1, e))
            });
        let gamma = i32::from_str_radix(&gamma, 2).unwrap();
        let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
        println!("{:?} ", gamma * epsilon);
    }
}
