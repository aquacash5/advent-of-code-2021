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

// fn main() {
//     let cli = Cli::from_args();
//     let file_lines = read_lines(cli.input);
//     if let Ok(lines) = file_lines {
//         let data: Vec<_> = lines.filter_map(Result::ok).collect();
//         let mut binary = vec![vec!['0'; 0]; data[0].len()];
//         for row in data {
//             for (i, col) in row.chars().enumerate() {
//                 binary[i].push(col)
//             }
//         }
//         let (gamma, epsilon) = binary
//             .into_iter()
//             .map(|c| c.into_iter().counts())
//             .map(|c| {
//                 if c[&'0'] > c[&'1'] {
//                     ('0', '1')
//                 } else {
//                     ('1', '0')
//                 }
//             })
//             .fold((String::from(""), String::from("")), |v, (g, e)| {
//                 (format!("{}{}", v.0, g), format!("{}{}", v.1, e))
//             });
//         let gamma = i32::from_str_radix(&gamma, 2).unwrap();
//         let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
//         println!("{:?} ", gamma * epsilon);
//     }
// }

fn common_for_column(it: &Vec<String>, col: usize) -> char {
    let data = it.into_iter().map(|s| s.chars().nth(col).unwrap()).counts();
    if data.get(&'0').unwrap_or(&0) > data.get(&'1').unwrap_or(&0) {
        '0'
    } else {
        '1'
    }
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<_> = lines.filter_map(Result::ok).collect();
        let mut temp_lines = data.clone();
        for col in 0..temp_lines[0].len() {
            let common = common_for_column(&temp_lines, col);
            temp_lines = temp_lines
                .into_iter()
                .filter(|s| s.chars().nth(col).unwrap() == common)
                .collect();
            if temp_lines.len() == 1 {
                break;
            }
        }
        let oxygen = i32::from_str_radix(&temp_lines[0], 2).unwrap();

        let mut temp_lines = data.clone();
        for col in 0..temp_lines[0].len() {
            let common = common_for_column(&temp_lines, col);
            temp_lines = temp_lines
                .into_iter()
                .filter(|s| s.chars().nth(col).unwrap() != common)
                .collect();
            if temp_lines.len() == 1 {
                break;
            }
        }
        let co2 = i32::from_str_radix(&temp_lines[0], 2).unwrap();

        println!("Oxygen: {:?}", oxygen);
        println!("CO2: {:?}", co2);

        println!("Life Support: {:?}", oxygen * co2);
    }
}
