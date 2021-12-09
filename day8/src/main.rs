use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day8", about = "Seven Segment Search")]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

const TWO_DECODE: [usize; 2] = [2, 5];
const THREE_DECODE: [usize; 3] = [0, 2, 5];
const FOUR_DECODE: [usize; 4] = [1, 2, 3, 5];
const FIVE_DECODE: [usize; 3] = [0, 3, 6];
const SIX_DECODE: [usize; 4] = [0, 1, 5, 6];
const NONE_DECODE: [usize; 0] = [];

const ZERO_DIGIT: u8 = 0b1110111u8;
const ONE_DIGIT: u8 = 0b0100100u8;
const TWO_DIGIT: u8 = 0b1011101u8;
const THREE_DIGIT: u8 = 0b1101101u8;
const FOUR_DIGIT: u8 = 0b0101110u8;
const FIVE_DIGIT: u8 = 0b1101011u8;
const SIX_DIGIT: u8 = 0b1111011u8;
const SEVEN_DIGIT: u8 = 0b0100101u8;
const EIGHT_DIGIT: u8 = 0b1111111u8;
const NINE_DIGIT: u8 = 0b1101111u8;

fn translate_digit(s: &str, m: &BTreeMap<char, u8>) -> char {
    let mut i = 0u8;
    for c in s.chars() {
        i ^= 2u8.pow(m[&c] as u32)
    }
    match i {
        ZERO_DIGIT => '0',
        ONE_DIGIT => '1',
        TWO_DIGIT => '2',
        THREE_DIGIT => '3',
        FOUR_DIGIT => '4',
        FIVE_DIGIT => '5',
        SIX_DIGIT => '6',
        SEVEN_DIGIT => '7',
        EIGHT_DIGIT => '8',
        NINE_DIGIT => '9',
        _ => 'a',
    }
}

#[derive(Debug, Clone)]
struct SegmentData {
    input: Vec<String>,
    output: Vec<String>,
    wiring: BTreeMap<char, u8>,
}

impl SegmentData {
    fn new(input: Vec<String>, output: Vec<String>) -> SegmentData {
        let mut new = SegmentData {
            input: input,
            output: output,
            wiring: BTreeMap::new(),
        };
        new.calculate_wiring();
        new
    }

    fn calculate_wiring(&mut self) {
        let mut options = vec![BTreeSet::from_iter("abcdefg".chars()); 7];
        for s in &self.input {
            let f: &[usize] = match s.len() {
                2 => &TWO_DECODE,
                3 => &THREE_DECODE,
                4 => &FOUR_DECODE,
                5 => &FIVE_DECODE,
                6 => &SIX_DECODE,
                _ => &NONE_DECODE,
            };
            for c in "abcdefg".chars().filter(|c| !s.contains(*c)) {
                for p in f {
                    options[*p].remove(&c);
                }
            }
        }
        while options.iter().any(|s| s.len() > 0) {
            for (i, s) in options.iter().enumerate().filter(|(_, s)| s.len() == 1) {
                self.wiring.insert(s.iter().fold('0', |_, c| *c), i as u8);
            }
            for c in options.iter_mut() {
                for (a, _) in self.wiring.iter() {
                    c.remove(&a);
                }
            }
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|s| translate_digit(s, &self.wiring))
            .collect()
    }
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
        let data: Vec<SegmentData> = lines
            .filter_map(Result::ok)
            .filter_map(|i| match i.split_once(" | ") {
                Some((a, b)) => Some((a.to_string(), b.to_string())),
                None => None,
            })
            .map(|(i, o)| {
                SegmentData::new(
                    i.split(" ").map(|s| s.to_string()).collect(),
                    o.split(" ").map(|s| s.to_string()).collect(),
                )
            })
            .collect();
        let part_1_filter = BTreeSet::from_iter([2, 4, 3, 7].iter());
        let part_1 = data
            .iter()
            .flat_map(|s| s.output.iter().filter(|i| part_1_filter.contains(&i.len())))
            .count();
        let part_2: u32 = data
            .iter()
            .map(|sd| sd.get_output().parse::<u32>().unwrap())
            .sum();

        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
