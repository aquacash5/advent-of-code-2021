use ndarray::Array2;
use queues::{queue, IsQueue, Queue};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day9", about = "Smoke Basin")]
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
fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let data: Vec<Vec<u32>> = lines
            .filter_map(Result::ok)
            .map(|s| s.chars().map(|i| i.to_digit(10).unwrap()).collect())
            .collect();
        let data: Array2<u32> = Array2::<u32>::from_shape_vec(
            (data.len(), data[0].len()),
            data.into_iter().flatten().collect(),
        )
        .unwrap();

        let lowest_points: Vec<(usize, usize)> = data
            .indexed_iter()
            .map(|((i, j), height)| {
                let up = data.get((i.wrapping_sub(1), j)).unwrap_or(&u32::MAX);
                let down = data.get((i.wrapping_add(1), j)).unwrap_or(&u32::MAX);
                let left = data.get((i, j.wrapping_sub(1))).unwrap_or(&u32::MAX);
                let right = data.get((i, j.wrapping_add(1))).unwrap_or(&u32::MAX);
                ([up, down, left, right].iter().all(|k| height < k), (i, j))
            })
            .filter(|(b, _)| *b)
            .map(|(_, h)| h)
            .collect();

        let part_1: u32 = lowest_points
            .iter()
            .filter_map(|p| data.get(*p))
            .map(|h| *h + 1)
            .sum();

        let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut sizes: Vec<usize> = vec![];
        for point in lowest_points.iter() {
            let mut size: usize = 0;
            let mut queue: Queue<(usize, usize)> = queue![*point];
            while let Ok((i, j)) = queue.remove() {
                if !seen.contains(&(i, j)) {
                    seen.insert((i, j));
                    if let Some(h) = data.get((i.saturating_sub(1), j)) {
                        if *h != 9 {
                            queue.add((i.saturating_sub(1), j)).unwrap();
                        }
                    }
                    if let Some(h) = data.get((i.saturating_add(1), j)) {
                        if *h != 9 {
                            queue.add((i.saturating_add(1), j)).unwrap();
                        }
                    }
                    if let Some(h) = data.get((i, j.saturating_sub(1))) {
                        if *h != 9 {
                            queue.add((i, j.saturating_sub(1))).unwrap();
                        }
                    }
                    if let Some(h) = data.get((i, j.saturating_add(1))) {
                        if *h != 9 {
                            queue.add((i, j.saturating_add(1))).unwrap();
                        }
                    }
                    size += 1;
                }
            }
            sizes.push(size);
        }
        sizes.sort_unstable();
        sizes.reverse();
        let part_2: usize = sizes.iter().take(3).product();

        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
