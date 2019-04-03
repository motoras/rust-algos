/**
 * Just be sure that you extract all the parent paths from a given path, and store them sorted.
 */
use std::cmp::min;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::stdin;
use std::io::*;

#[derive(Debug, PartialEq, Eq)]
struct Path<'a> {
    components: &'a [&'a str],
}

impl<'a> Path<'a> {
    fn new(path: &'a Vec<&str>, index: &mut BTreeSet<Path<'a>>) {
        for i in 0..path.len() {
            let slice = &path[0..(i + 1)];
            index.insert(Path { components: slice });
        }
    }
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut spaces = String::new();
        for i in 0..self.components.len() {
            if i > 0 {
                spaces.push(' ');
            }
        }
        write!(f, "{}{}", spaces, self.components.last().unwrap())
    }
}

impl<'a> PartialOrd for Path<'a> {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Path<'a> {
    fn cmp(&self, other: &Path) -> Ordering {
        let min_len = min(self.components.len(), other.components.len());
        for i in 0..min_len {
            let res = self.components[i].cmp(&other.components[i]);
            if res != Ordering::Equal {
                return res;
            }
        }
        self.components.len().cmp(&other.components.len())
    }
}
fn main() {
    let stdin = stdin();
    let mut paths_count_line = String::new();
    stdin.read_line(&mut paths_count_line).unwrap();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|line| line.ok()).collect();
    let splits: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split("\\").collect())
        .collect();
    let mut index = BTreeSet::<Path>::new();
    for split in &splits {
        Path::new(split, &mut index);
    }
    for p in index.iter() {
        println!("{}", p);
    }
}
