// use itertools::Itertools;
use itertools::Itertools;
use std::collections::BTreeSet;
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

fn get_column(board: &[[i32; 5]; 5], nth: usize) -> [i32; 5] {
    let mut col = [0; 5];
    for (i, row) in board.iter().enumerate() {
        col[i] = row[nth];
    }
    col
}

fn score_board(board: &[[i32; 5]; 5], set: &BTreeSet<i32>) -> i32 {
    board.iter().flatten().filter(|i| !set.contains(i)).sum()
}

fn find_winning_board(boards: &Vec<[[i32; 5]; 5]>, results: &Vec<i32>, print: bool) -> usize {
    let mut result_set: BTreeSet<i32> = BTreeSet::new();

    for result_pos in 0..results.len() {
        result_set.insert(results[result_pos]);
        for (board_number, board) in boards.iter().enumerate() {
            for row in board {
                if row.iter().all(|i| result_set.contains(i)) {
                    let score = score_board(&board, &result_set);
                    if print {
                        println!("Results: {:?}", result_set);
                        println!("Row: {:?}", row);
                        println!("Last Value: {:?}", results[result_pos]);
                        println!("Score: {:?}", score);
                        println!("Final Score: {:?}", score * results[result_pos]);
                    }
                    return board_number;
                }
            }
            for col_num in 0..5 {
                let col = get_column(&board, col_num);
                if col.iter().all(|i| result_set.contains(i)) {
                    let score = score_board(&board, &result_set);
                    if print {
                        println!("Results: {:?}", result_set);
                        println!("Column: {:?}", col);
                        println!("Last Value: {:?}", results[result_pos]);
                        println!("Score: {:?}", score);
                        println!("Final Score: {:?}", score * results[result_pos]);
                    }
                    return board_number;
                }
            }
        }
    }
    return 0;
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<_> = lines.filter_map(Result::ok).collect();
        let results: Vec<i32> = data[0]
            .split(&",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut boards: Vec<[[i32; 5]; 5]> = Vec::new();
        for board in &data.into_iter().skip(1).chunks(6) {
            let mut v = [[0; 5]; 5];
            for (i, row) in board.skip(1).enumerate() {
                let items = row.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                for (j, col) in items.enumerate() {
                    v[i][j] = col;
                }
            }
            boards.push(v);
        }
        // Part 2
        // for _ in 1..boards.len() {
        //     boards.remove(find_winning_board(&boards, &results, false));
        // }
        find_winning_board(&boards, &results, true);
    }
}
