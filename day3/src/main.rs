use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", about = "Binary Diagnostic")]
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

fn common_for_column(it: &Vec<String>, col: usize) -> char {
    let data = it.into_iter().map(|s| s.chars().nth(col).unwrap()).counts();
    if data.get(&'0').unwrap_or(&0) > data.get(&'1').unwrap_or(&0) {
        '0'
    } else {
        '1'
    }
}

fn calculate_gamma(it: &Vec<String>) -> String {
    (0..it[0].len())
        .map(|col| common_for_column(it, col))
        .collect()
}

fn calculate_epsilon(it: &Vec<String>) -> String {
    (0..it[0].len())
        .map(|col| {
            if common_for_column(it, col) == '0' {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn find_oxy(it: &Vec<String>) -> String {
    fn find_oxy_part(it: &Vec<String>, col: usize) -> String {
        let common = common_for_column(it, col);
        let new_it: Vec<String> = it
            .into_iter()
            .cloned()
            .filter(|s| s.chars().nth(col).unwrap() == common)
            .collect();
        if new_it.len() <= 1 {
            new_it[0].clone()
        } else {
            find_oxy_part(&new_it, col + 1)
        }
    }
    find_oxy_part(it, 0)
}

fn find_co2(it: &Vec<String>) -> String {
    fn find_co2_part(it: &Vec<String>, col: usize) -> String {
        let common = common_for_column(it, col);
        let new_it: Vec<String> = it
            .into_iter()
            .cloned()
            .filter(|s| s.chars().nth(col).unwrap() != common)
            .collect();
        if new_it.len() <= 1 {
            new_it[0].clone()
        } else {
            find_co2_part(&new_it, col + 1)
        }
    }
    find_co2_part(it, 0)
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<_> = lines.filter_map(Result::ok).collect();

        let gamma = i32::from_str_radix(&calculate_gamma(&data), 2).unwrap();
        let epsilon = i32::from_str_radix(&calculate_epsilon(&data), 2).unwrap();

        let oxygen = i32::from_str_radix(&find_oxy(&data), 2).unwrap();
        let co2 = i32::from_str_radix(&find_co2(&data), 2).unwrap();

        println!("Part 1: {:?}", gamma * epsilon);
        println!("Part 2: {:?}", oxygen * co2);
    }
}
