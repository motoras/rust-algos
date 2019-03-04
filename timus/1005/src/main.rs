/**
 * Just brute force. Generate all the possible permuations(actually half are enough) and check which one is the best.
 */
use std::cmp::min;
fn solve_brute_force(stones: &[usize]) -> usize {
    let stones_count = stones.len();
    let permutations = 1 << stones_count;
    let mut total = 0;
    for stone_weight in stones.iter() {
        total += stone_weight;
    }
    let mut best = std::usize::MAX;
    for crt_perm in 0..permutations / 2 {
        let mut crt_total = 0;
        for i in 0..stones_count {
            if crt_perm & (1 << i) != 0 {
                crt_total += stones[i];
            }
        }
        let left = total - crt_total;
        if left > crt_total {
            best = min(best, left - crt_total)
        } else {
            best = min(best, crt_total - left)
        }
        if best == 0 {
            return 0;
        }
    }
    best
}

use std::io::stdin;
fn main() {
    let reader = stdin();
    let mut stones_count_line = String::new();
    reader.read_line(&mut stones_count_line).unwrap();
    let stones_count: usize = stones_count_line.trim_right().parse().unwrap();

    let mut stones_line = String::new();
    reader.read_line(&mut stones_line).unwrap();
    let stones_data: Vec<&str> = stones_line.trim_right().split(' ').collect();
    let mut stones: [usize; 20] = [0; 20];
    for i in 0..stones_count {
        stones[i] = stones_data[i].parse().unwrap();
    }
    let res = solve_brute_force(&stones[0..stones_count]);
    println!("{}", res);
}
