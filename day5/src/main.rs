use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day5", about = "Hydrothermal Venture")]
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
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn max_value(&self) -> i32 {
        cmp::max(self.x, self.y)
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let p_strings: Vec<&str> = s.split(",").collect();
        Point {
            y: p_strings[0].parse().unwrap(),
            x: p_strings[1].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    start: Point,
    end: Point,
}

impl Pair {
    fn match_x(&self) -> bool {
        self.start.x == self.end.x
    }

    fn match_y(&self) -> bool {
        self.start.y == self.end.y
    }

    fn max_value(&self) -> i32 {
        cmp::max(self.start.max_value(), self.end.max_value())
    }
}

impl From<String> for Pair {
    fn from(s: String) -> Self {
        let p_strings: Vec<&str> = s.split(" -> ").collect();
        Pair {
            start: Point::from(p_strings[0]),
            end: Point::from(p_strings[1]),
        }
    }
}

fn unit(i: i32) -> i32 {
    i / i.abs()
}

fn create_field_map(size: usize, data: &Vec<Pair>) -> Vec<u32> {
    let mut field = vec![0u32; size * size];
    let i_size = size as i32;
    for pair in data {
        if pair.match_x() {
            let (s, e) = if pair.start.y > pair.end.y {
                (pair.end, pair.start)
            } else {
                (pair.start, pair.end)
            };

            for y in s.y..(e.y + 1) {
                field[(pair.start.x * i_size + y) as usize] += 1;
            }
        } else if pair.match_y() {
            let (s, e) = if pair.start.x > pair.end.x {
                (pair.end, pair.start)
            } else {
                (pair.start, pair.end)
            };

            for x in s.x..(e.x + 1) {
                field[(x * i_size + pair.start.y) as usize] += 1;
            }
        } else {
            let (s, e) = if pair.start.x > pair.end.x {
                (pair.start, pair.end)
            } else {
                (pair.end, pair.start)
            };
            let slop = unit(e.y - s.y);

            for step in 0..((s.x - e.x) + 1) {
                field[((s.x + (step * -1)) * i_size + (s.y + (step * slop))) as usize] += 1;
            }
        }
    }
    field
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let part_2_data: Vec<_> = lines
            .filter_map(Result::ok)
            .map(|i| Pair::from(i))
            .collect();
        let part_1_data: Vec<_> = part_2_data
            .iter()
            .cloned()
            .filter(|p| p.match_x() || p.match_y())
            .collect();
        let size: usize = (part_2_data.iter().map(|p| p.max_value()).max().unwrap() + 1) as usize;
        let part_1 = create_field_map(size, &part_1_data)
            .into_iter()
            .filter(|i| *i > 1)
            .count();
        let part_2 = create_field_map(size, &part_2_data)
            .into_iter()
            .filter(|i| *i > 1)
            .count();
        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
