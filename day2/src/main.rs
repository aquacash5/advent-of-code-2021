use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day2", about = "Dive!")]
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

#[derive(Debug, Clone, Copy)]
struct Location {
    depth: i32,
    position: i32,
    aim: i32,
}

impl Location {
    fn next_location(&self, instruction: &String) -> Location {
        let data = instruction.split_once(' ').unwrap();
        let i = data.0;
        let d = data.1.parse::<i32>().unwrap();
        match i {
            "forward" => Location {
                depth: self.depth + d,
                position: self.position,
                aim: self.aim,
            },
            "down" => Location {
                depth: self.depth,
                position: self.position + d,
                aim: self.aim,
            },
            "up" => Location {
                depth: self.depth,
                position: self.position - d,
                aim: self.aim,
            },
            _ => *self,
        }
    }

    fn next_location_aim(&self, instruction: &String) -> Location {
        let data = instruction.split_once(' ').unwrap();
        let i = data.0;
        let d = data.1.parse::<i32>().unwrap();
        match i {
            "forward" => Location {
                depth: self.depth + (self.aim * d),
                position: self.position + d,
                aim: self.aim,
            },
            "down" => Location {
                depth: self.depth,
                position: self.position,
                aim: self.aim + d,
            },
            "up" => Location {
                depth: self.depth,
                position: self.position,
                aim: self.aim - d,
            },
            _ => *self,
        }
    }
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    let init_loc = Location {
        depth: 0,
        position: 0,
        aim: 0,
    };
    if let Ok(lines) = file_lines {
        let data: Vec<String> = lines.filter_map(Result::ok).collect();

        let part_1 = data
            .iter()
            .fold(init_loc, |cur, line| cur.next_location(line));
        let part_2 = data
            .iter()
            .fold(init_loc, |cur, line| cur.next_location_aim(line));

        println!("Part 1: {:?}", part_1.depth * part_1.position);
        println!("Part 2: {:?}", part_2.depth * part_2.position);
    }
}
