use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LEN: usize = 80;

fn main() {
    let mut input = [[0u32; LEN]; LEN];
    // input[0] =[131,673,234,103,18];
    // input[1] =[201,96,342,965,150];
    // input[2] =[630,803,746,422,111];
    // input[3] =[537,699,497,121,956];
    // input[4] =[805,732,524,37,331];

    let file = File::open("p081_matrix.txt").unwrap();
    for (idx, line) in BufReader::new(file).lines().enumerate() {
        let mx_line: Vec<u32> = line
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        assert!(mx_line.len() == LEN);
        input[idx].copy_from_slice(&mx_line[..LEN]);
        //print!("{} {}", idx, line.unwrap());
    }
    let mut dp = [[0u32; LEN]; LEN];
    for i in 0..LEN {
        for j in 0..LEN {
            dp[i][j] = input[i][j];
            if i > 0 {
                if j > 0 {
                    dp[i][j] += min(dp[i - 1][j], dp[i][j - 1]);
                } else {
                    dp[i][j] += dp[i - 1][j];
                }
            } else {
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
    }
    println!("{}", dp[LEN - 1][LEN - 1]);
}
