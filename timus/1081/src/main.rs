/**
 *  While you can check every possible number the brute force will run again out of time.
 *  There is a recursive solution that can be discovered, if for evey length you separetly
 *  count valid solution starting with zero and starting with one.
 */
use std::cmp::min;
use std::io::stdin;
use std::io::Read;

fn find_seq_dp(number_len: usize, seq_number: usize) -> String {
    let mut dp = [[0usize; 2]; 44];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..number_len {
        dp[i][0] = min(dp[i - 1][0] + dp[i - 1][1], seq_number); //more than we care so avoid overflow
        dp[i][1] = dp[i - 1][0];
    }
    if dp[number_len - 1][0] + dp[number_len - 1][1] < seq_number {
        return String::from("-1");
    }
    let mut res = String::with_capacity(number_len);
    let mut k = seq_number;
    for i in (0..number_len).rev() {
        if dp[i][0] >= k {
            res.push('0');
        } else {
            res.push('1');
            k -= dp[i][0];
        }
    }
    res
}

fn main() {
    let mut input_line = String::with_capacity(16);
    stdin().read_to_string(&mut input_line).unwrap();
    stdin().read_line(&mut input_line).unwrap();
    let mut res = input_line.split_whitespace();
    let num_len: usize = res.next().unwrap().parse().unwrap();
    let seq_num: usize = res.next().unwrap().parse().unwrap();
    println!("{}", find_seq_dp(num_len, seq_num));
}
