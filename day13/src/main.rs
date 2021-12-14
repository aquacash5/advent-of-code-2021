use ndarray::{s, Array, Array2, ArrayView2};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day13", about = "Transparent Origami")]
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
enum FoldDirection {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct PaperFold {
    direction: FoldDirection,
    position: usize,
}

impl PaperFold {
    fn new(s: String) -> PaperFold {
        use FoldDirection::*;
        let drop = s.chars().skip(11).collect::<String>();
        let (dir, num) = drop.split_once("=").unwrap();
        PaperFold {
            direction: if dir == "x" { X } else { Y },
            position: num.parse().unwrap(),
        }
    }

    fn paper_fold(&self, arr: Array2<u8>) -> Array2<u8> {
        use FoldDirection::*;
        match self.direction {
            X => {
                let top = &arr.slice(s![..self.position, ..]);
                let bottom = &arr.slice(s![self.position+1..;-1, ..]);
                top + bottom
            }
            Y => {
                let top = &arr.slice(s![.., ..self.position]);
                let bottom = &arr.slice(s![.., self.position+1..;-1]);
                top + bottom
            }
        }
    }
}

fn print_grid(grid: &ArrayView2<u8>) {
    for col in grid.columns() {
        println!(
            "{}",
            col.iter()
                .map(|b| if *b > 0 { "█" } else { " " })
                .collect::<String>()
        );
    }
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let mut data = lines.filter_map(Result::ok);
        let points: Vec<(usize, usize)> = data
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split_once(",")
                    .and_then(|(x, y)| Some((x.parse().unwrap(), y.parse().unwrap())))
                    .unwrap()
            })
            .collect();
        let x_max = points.iter().map(|(x, _)| x).max().unwrap().to_owned();
        let y_max = points.iter().map(|(_, y)| y).max().unwrap().to_owned();
        let mut grid: Array2<u8> = Array::from_shape_fn((x_max + 1, y_max + 1), |_| 0);
        for point in points {
            *grid.get_mut(point).unwrap() = 1;
        }
        let folds: Vec<_> = data.map(PaperFold::new).collect();
        let part_1 = folds[0]
            .paper_fold(grid.clone())
            .iter()
            .filter(|i| **i > 0)
            .count();
        let part_2 = &folds.iter().fold(grid, |g, f| f.paper_fold(g));
        println!("Part 1: {:?}", part_1);
        println!("Part 2:");
        print_grid(&part_2.view());
    }
}
