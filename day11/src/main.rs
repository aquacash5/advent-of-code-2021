use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day11", about = "Dumbo Octopus")]
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

const GRID_SIZE: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Octopus {
    energy_level: u8,
    flashes: u32,
    flashed: bool,
}

impl Octopus {
    const MAX_ENERGY: u8 = 9;

    fn new(energy_level: u8) -> Octopus {
        Octopus {
            energy_level,
            flashes: 0,
            flashed: false,
        }
    }

    fn charge(&mut self) -> bool {
        if !self.flashed {
            self.energy_level += 1;
        }
        if self.energy_level > Octopus::MAX_ENERGY {
            self.flashes += 1;
            self.flashed = true;
            self.energy_level = 0;
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.flashed = false;
    }
}

fn flash_grid(grid: &mut Array2<Octopus>, (x, y): (usize, usize)) {
    let x_a = x.checked_add(1);
    let x_s = x.checked_sub(1);
    let y_a = y.checked_add(1);
    let y_s = y.checked_sub(1);
    let x_0 = Some(x);
    let y_0 = Some(y);

    #[rustfmt::skip]
    let neighbors: Vec<_> = [
        (x_s, y_s), (x_s, y_0), (x_s, y_a),
        (x_0, y_s), /*Center*/  (x_0, y_a),
        (x_a, y_s), (x_a, y_0), (x_a, y_a),
    ]
    .iter()
    .filter_map(|o_pair| match o_pair {
        (Some(x), Some(y)) => Some((*x, *y)),
        _ => None,
    })
    .collect();

    for c_point in neighbors {
        if grid.get_mut(c_point).map(Octopus::charge).unwrap_or(false) {
            flash_grid(grid, c_point);
        }
    }
}

fn step_grid(grid: &mut Array2<Octopus>) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid.get_mut((x, y)).map(Octopus::charge).unwrap_or(false) {
                flash_grid(grid, (x, y));
            }
        }
    }
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Array2<Octopus> = Array2::from_shape_vec(
            (GRID_SIZE, GRID_SIZE),
            lines
                .filter_map(Result::ok)
                .flat_map(|s| s.chars().collect::<Vec<char>>())
                .map(|c| Octopus::new(c.to_digit(10).unwrap() as u8))
                .collect(),
        )
        .unwrap();

        let mut mut_grid = data.clone();
        for _ in 0..100 {
            step_grid(&mut mut_grid);
            mut_grid.map_mut(Octopus::reset);
        }
        let part_1: u32 = mut_grid.iter().map(|o| o.flashes).sum();

        let mut mut_grid = data;
        let mut part_2: u32 = 0;
        while mut_grid.iter().map(|o| o.energy_level as u32).sum::<u32>() != 0 {
            step_grid(&mut mut_grid);
            mut_grid.map_mut(Octopus::reset);
            part_2 += 1;
        }

        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
