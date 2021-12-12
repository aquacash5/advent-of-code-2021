use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Day12", about = "Passage Pathing")]
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

#[derive(Debug, Clone)]
struct Node {
    children: BTreeSet<String>,
    seen: bool,
    tracked: bool,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            children: BTreeSet::new(),
            seen: false,
            tracked: name.chars().all(char::is_lowercase),
        }
    }

    fn append(&mut self, child: String) {
        if child != "start" {
            self.children.insert(child);
        }
    }

    fn mark(&mut self) {
        self.seen = true;
    }

    fn marked(&self) -> bool {
        self.seen && self.tracked
    }

    fn iter(&self) -> impl Iterator<Item = &String> + '_ {
        self.children.iter()
    }
}

#[derive(Debug, Clone)]
struct Graph {
    map: BTreeMap<String, Node>,
    marked: bool,
}
impl Graph {
    fn new() -> Self {
        Graph {
            map: BTreeMap::new(),
            marked: false,
        }
    }

    fn add(&mut self, con_str: String) {
        let (left, right) = con_str.split_once("-").unwrap();
        let left = left.to_string();
        let right = right.to_string();
        self.map
            .entry(left.clone())
            .or_insert(Node::new(left.clone()))
            .append(right.clone());
        self.map
            .entry(right.clone())
            .or_insert(Node::new(right.clone()))
            .append(left.clone());
    }

    fn get(&self, name: &String) -> Option<&Node> {
        self.map.get(name).and_then(|n| {
            if n.marked() && self.marked {
                None
            } else {
                Some(n)
            }
        })
    }

    fn get_mut(&mut self, name: &String) -> Option<&mut Node> {
        self.map.get_mut(name)
    }
}

fn count_routes(graph: &Graph) -> usize {
    fn count_routes_inner(mut i_graph: Graph, name: String) -> usize {
        let mut count: usize = 0;
        let children: Option<Vec<String>> = i_graph.get(&name).map(|n| n.iter().cloned().collect());
        if let Some(children) = children {
            let cur = i_graph.get(&name).unwrap();
            if cur.marked() {
                i_graph.marked = true;
            }
            i_graph.get_mut(&name).unwrap().mark();
            for child in children {
                match child.as_str() {
                    "end" => count += 1,
                    _ => count += count_routes_inner(i_graph.clone(), child),
                }
            }
        }
        count
    }
    count_routes_inner(graph.clone(), "start".to_string())
}

fn main() {
    let cli = Cli::from_args();
    let file_lines = read_lines(cli.input);
    if let Ok(lines) = file_lines {
        let mut data: Graph = lines.filter_map(Result::ok).fold(Graph::new(), |mut g, s| {
            g.add(s);
            g
        });
        data.marked = true;
        let part_1 = count_routes(&data);
        data.marked = false;
        let part_2 = count_routes(&data);
        println!("Part 1: {:?}", part_1);
        println!("Part 2: {:?}", part_2);
    }
}
