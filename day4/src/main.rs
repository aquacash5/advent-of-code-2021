use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const BINGO_SIZE: usize = 5;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day4", about = "Giant Squid")]
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

fn get_column(board: &[[u32; BINGO_SIZE]; BINGO_SIZE], nth: usize) -> [u32; BINGO_SIZE] {
    let mut col = [0; BINGO_SIZE];
    for (i, row) in board.iter().enumerate() {
        col[i] = row[nth];
    }
    col
}

fn score_board(board: &[[u32; BINGO_SIZE]; BINGO_SIZE], set: &BTreeSet<u32>) -> u32 {
    board.iter().flatten().filter(|i| !set.contains(i)).sum()
}

fn find_winning_board(
    boards: &Vec<[[u32; BINGO_SIZE]; BINGO_SIZE]>,
    results: &Vec<u32>,
) -> (usize, u32) {
    let mut result_set: BTreeSet<u32> = BTreeSet::new();

    for result_pos in 0..results.len() {
        result_set.insert(results[result_pos]);
        for (board_number, board) in boards.iter().enumerate() {
            for row in board {
                if row.iter().all(|i| result_set.contains(i)) {
                    let score = score_board(&board, &result_set);
                    return (board_number, score * results[result_pos]);
                }
            }
            for col_num in 0..BINGO_SIZE {
                let col = get_column(&board, col_num);
                if col.iter().all(|i| result_set.contains(i)) {
                    let score = score_board(&board, &result_set);
                    return (board_number, score * results[result_pos]);
                }
            }
        }
    }
    return (0, 0);
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<_> = lines.filter_map(Result::ok).collect();
        let results: Vec<u32> = data[0]
            .split(&",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let mut boards: Vec<[[u32; BINGO_SIZE]; BINGO_SIZE]> = Vec::new();
        for board in &data.into_iter().skip(1).chunks(6) {
            let mut v = [[0; BINGO_SIZE]; BINGO_SIZE];
            for (i, row) in board.skip(1).enumerate() {
                let items = row.split_whitespace().map(|s| s.parse::<u32>().unwrap());
                for (j, col) in items.enumerate() {
                    v[i][j] = col;
                }
            }
            boards.push(v);
        }
        let (_, part_1) = find_winning_board(&boards, &results);
        // Part 2
        for _ in 1..boards.len() {
            let (win_board, _) = find_winning_board(&boards, &results);
            boards.remove(win_board);
        }
        let (_, part_2) = find_winning_board(&boards, &results);
        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
