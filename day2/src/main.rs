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

#[derive(Debug)]
struct Location {
    depth: i32,
    position: i32,
}

impl Location {
    fn next_location(&self, instruction: String) -> Location {
        let data = instruction.split_once(' ').unwrap();
        let i = data.0;
        let d = data.1.parse::<i32>().unwrap();
        match i {
            "forward" => Location {
                depth: self.depth + d,
                position: self.position,
            },
            "down" => Location {
                depth: self.depth,
                position: self.position + d,
            },
            "up" => Location {
                depth: self.depth,
                position: self.position - d,
            },
            _ => Location {
                depth: self.depth,
                position: self.position,
            },
        }
    }
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    let init_loc = Location {
        depth: 0,
        position: 0,
    };
    if let Ok(lines) = file_lines {
        let data: Location = lines
            .filter_map(Result::ok)
            .fold(init_loc, |cur, line| cur.next_location(line));
        println!("{:?}", data.depth * data.position)
    }
}
