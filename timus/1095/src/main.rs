/**
 * Just find seven permuation of 1234, one which is 0 mod 7, one which is 1 mod 7 up to 6 mod  7.
 * Take the number whithout  a '1', '2', '3', '4', be creful of trailnig zeroes, compute it's value
 * mod 7 and append the 1234 permutaion wich complements it.
 */
use std::io::stdin;

#[inline]
fn pow_10_mod_7(n: usize) -> usize {
    let mut res = 1;
    for _i in 0..n {
        res = (res * 10) % 7;
    }
    res
}

#[inline]
fn divisible_by_seven_str(number: &str) -> String {
    let mod_7_perm = ["4123", "1324", "1234", "4231", "1432", "2413", "2134"];
    let mut found = [false; 4];
    let mut candidate = 0;
    let mut extra_zeroes = 0;
    let mut res = String::new();
    for crt_char in number.chars() {
        let crt_digit = crt_char.to_digit(10).unwrap() as usize;
        if crt_digit > 0 && crt_digit <= 4 && !found[crt_digit - 1] {
            found[crt_digit - 1] = true;
        } else if crt_digit == 0 {
            extra_zeroes += 1;
        } else {
            res.push(crt_char);
            candidate = (candidate * 10 + crt_digit) % 7;
        }
    }
    if res.len() != 0 {
        let rem = (candidate % 7 * (pow_10_mod_7(4 + extra_zeroes))) % 7;
        let to_join = mod_7_perm[(7 - rem) % 7];
        for _i in 0..extra_zeroes {
            res.push_str("0");
        }
        res.push_str(to_join);
        res
    } else {
        let mut res = mod_7_perm[0].to_string();
        for _i in 0..extra_zeroes {
            res.push_str("0");
        }
        res
    }
}

fn main() {
    let reader = stdin();
    let mut tests_count_line = String::new();
    reader.read_line(&mut tests_count_line).unwrap();
    let tests_count: usize = tests_count_line.trim_right().parse().unwrap();
    for _i in 0..tests_count {
        let mut nr_line = String::new();
        reader.read_line(&mut nr_line).unwrap();
        println!("{}", divisible_by_seven_str(nr_line.trim_right()));
    }
}
