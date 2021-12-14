use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::AddAssign;
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

fn build_polymer(
    polymer: String,
    tree: &BTreeMap<(char, char), ((char, char), (char, char))>,
    iterations: usize,
) -> BTreeMap<char, usize> {
    let mut accumulator: BTreeMap<(char, char), usize> = BTreeMap::new();
    for p in polymer.chars().tuple_windows() {
        accumulator.entry(p).or_insert(0).add_assign(1);
    }
    for _ in 0..iterations {
        accumulator = accumulator.iter().fold(BTreeMap::new(), |mut n, (i, s)| {
            let (a, b) = tree.get(&i).unwrap();
            n.entry(*a).or_insert(0).add_assign(s);
            n.entry(*b).or_insert(0).add_assign(s);
            n
        });
    }
    let mut accumulator: BTreeMap<char, usize> =
        accumulator
            .iter()
            .fold(BTreeMap::new(), |mut n, ((a, b), s)| {
                n.entry(*a).or_insert(0).add_assign(s);
                n.entry(*b).or_insert(0).add_assign(s);
                n
            });
    accumulator
        .entry(polymer.chars().next().unwrap())
        .or_insert(0)
        .add_assign(1);
    accumulator
        .entry(polymer.chars().rev().next().unwrap())
        .or_insert(0)
        .add_assign(1);
    accumulator.iter_mut().for_each(|(_, s)| *s /= 2);
    accumulator
}

fn solution(counts: &BTreeMap<char, usize>) -> usize {
    let max_char_count = counts.clone().into_values().max().unwrap();
    let min_char_count = counts.clone().into_values().min().unwrap();
    max_char_count - min_char_count
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let mut data = lines.filter_map(Result::ok);
        let polymer: String = data.next().unwrap().clone();
        let data: BTreeMap<(char, char), ((char, char), (char, char))> =
            data.skip(1).fold(BTreeMap::new(), |mut tree, s| {
                tree.insert(
                    (s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()),
                    (
                        (s.chars().nth(0).unwrap(), s.chars().nth(6).unwrap()),
                        (s.chars().nth(6).unwrap(), s.chars().nth(1).unwrap()),
                    ),
                );
                tree
            });
        let part_1 = solution(&build_polymer(polymer.clone(), &data, 10));
        let part_2 = solution(&build_polymer(polymer.clone(), &data, 40));
        println!("Part 1: {}", part_1);
        println!("Part 2: {}", part_2);
    }
}
