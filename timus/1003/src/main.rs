use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
/**
 * Basically keep a list with the existign intervals, and split them  when a new one overlaps
 * over existing one.
 */
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Debug)]
struct Cond {
    from: usize,
    to: usize,
    even: bool,
}
impl Cond {
    fn new(from: usize, to: usize, even: bool) -> Self {
        Cond { from, to, even }
    }
}
#[inline]
fn solve(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);
    loop {
        let mut len_line = String::new();
        reader.read_line(&mut len_line).unwrap();
        let array_len: i32 = len_line.trim_right().parse().unwrap();
        if array_len == -1 {
            break;
        }
        let mut conds_count_line = String::new();
        reader.read_line(&mut conds_count_line).unwrap();
        let conds_count: usize = conds_count_line.trim_right().parse().unwrap();
        if conds_count == 0 {
            writeln!(output, "0").unwrap();
            continue;
        }
        let mut data = HashMap::<usize, Cond>::with_capacity(conds_count * 2);

        let mut done = false;
        for i in 0..conds_count {
            let mut cond_line = String::new();
            reader.read_line(&mut cond_line).unwrap();
            if done {
                continue;
            }

            let cond_data: Vec<&str> = cond_line.trim_right().split(' ').collect();
            let mut from: usize = cond_data[0].parse().unwrap();
            let mut to: usize = cond_data[1].parse().unwrap();
            let mut even = cond_data[2] == "even";
            loop {
                match data.entry(to) {
                    Vacant(entry) => {
                        entry.insert(Cond::new(from, to, even));
                        break;
                    }
                    Occupied(mut entry) => {
                        let crt_cond = entry.get_mut();
                        if crt_cond.from == from {
                            if crt_cond.even != even {
                                writeln!(output, "{}", i).unwrap();
                                done = true;
                            }
                            break;
                        } else if crt_cond.from > from {
                            to = crt_cond.from - 1;
                            even = even == crt_cond.even;
                        } else {
                            let new_cond = Cond::new(from, to, even);
                            to = from - 1;
                            from = crt_cond.from;
                            even = even == crt_cond.even;
                            crt_cond.from = new_cond.from;
                            crt_cond.even = new_cond.even;
                        }
                    }
                };
            }
        }
        if !done {
            writeln!(output, "{}", conds_count).unwrap();
        }
    }
}

use std::io::stdin;
use std::io::stdout;
fn main() {
    solve(&mut stdin(), &mut stdout());
}
