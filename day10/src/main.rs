use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day10", about = "Syntax Scoring")]
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

lazy_static! {
    static ref BAD_SCORE_TABLE: BTreeMap<char, u64> =
        BTreeMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    static ref FIX_SCORE_TABLE: BTreeMap<char, u64> =
        BTreeMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    static ref MATCH_TABLE: BTreeMap<char, char> =
        BTreeMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    static ref START_CHARS: BTreeSet<char> = BTreeSet::from(['(', '[', '{', '<']);
    static ref END_CHARS: BTreeSet<char> = BTreeSet::from([')', ']', '}', '>']);
}

fn find_bad_end(run: &String) -> Option<char> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in run.chars() {
        if START_CHARS.contains(&c) {
            stack.push_back(c);
            continue;
        }
        if END_CHARS.contains(&c) {
            if let Some(prev) = stack.pop_back() {
                if let Some(m) = MATCH_TABLE.get(&prev) {
                    if *m == c {
                        continue;
                    }
                }
            }
        }
        return Some(c);
    }
    None
}

#[allow(dead_code)]
fn get_corrected_endings(run: &String) -> Option<String> {
    let mut stack: VecDeque<char> = VecDeque::new();
    for c in run.chars() {
        if START_CHARS.contains(&c) {
            stack.push_back(c);
            continue;
        }
        if END_CHARS.contains(&c) {
            if let Some(prev) = stack.pop_back() {
                if let Some(m) = MATCH_TABLE.get(&prev) {
                    if *m == c {
                        continue;
                    }
                }
            }
        }
        return None;
    }
    let mut r_str = String::new();
    for c in stack.iter().rev() {
        if let Some(m) = MATCH_TABLE.get(c) {
            r_str += &m.to_string();
        }
    }
    Some(r_str)
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<_> = lines.filter_map(Result::ok).collect();
        let part_1: u64 = data
            .iter()
            .filter_map(|s| find_bad_end(s))
            .filter_map(|c| BAD_SCORE_TABLE.get(&c))
            .sum();

        let part_2: Vec<u64> = data
            .iter()
            .filter_map(|s| {
                if find_bad_end(s).is_some() {
                    None
                } else {
                    Some(s)
                }
            })
            .filter_map(|s| get_corrected_endings(s))
            .map(|s| {
                s.chars().fold(0_u64, |i, c| {
                    (5 * i) + FIX_SCORE_TABLE.get(&c).unwrap_or(&0)
                })
            })
            .sorted()
            .collect();

        let part_2 = part_2[part_2.len() / 2];

        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
