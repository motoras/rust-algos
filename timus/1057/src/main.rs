/**
 * We are searching for numbers which a given base b, have exactly k 1's and all other digits zero.
 * To check if a number match this condition, could be done by simply converting it  in a given base
 * and look at it's digits. Howwever the intervals could be quite large and such a solution will run
 * out of time, so we must find a way to count themwithout checking every given number.
 * First observation is that F(X,Y) = F(1,Y) - F(1,X-1) so we only need to count all such numbers from
 * 1 to a given bounf X. Second observation is that All the number in base B no longer than N digits which
 * satisfy the problem's conditions are the number of K-combinations by N. In order to count all such numbers
 * we apply the following rules:
 *              - if numbers starts with a digit greater than 1 than we simply apply the combination formula
 *              - if the number starts with 1, we remove that digit and count recursivley but for k-1
 *              - if a number starts with 0, we search we remove that digit and count recursivley
 *
 */
use std::cmp::max;
use std::io::stdin;

#[inline]
fn to_base(n: usize, base: usize) -> Vec<usize> {
    let mut digits_b = Vec::<usize>::with_capacity(digits_count(n, base));
    let mut val = n;
    while val != 0 {
        let b_digit = val % base;
        val = val / base;
        digits_b.push(b_digit);
    }
    digits_b.reverse();
    digits_b
}

#[inline]
fn digits_count(n: usize, base: usize) -> usize {
    let division = ((n as f64).ln()) / ((base as f64).ln());
    division.floor() as usize + 1
}

#[inline]
fn comb_n_by_k(n: usize, kk: usize) -> usize {
    if kk > n {
        return 0; //we need this it may happen
    }
    if kk == 0 || kk == n {
        return 1;
    }
    let mut res = 1;
    let mut simplify_by = 2;
    let k = max(kk, n - kk);
    let mut step = n - k + 1;
    while step <= n {
        let mut crt = step;
        while simplify_by <= k && crt % simplify_by == 0 {
            crt = crt / simplify_by;
            simplify_by += 1;
        }
        res *= crt;
        while simplify_by <= k && res % simplify_by == 0 {
            res = res / simplify_by;
            simplify_by += 1;
        }
        step += 1;
    }
    res
}

#[inline]
fn amount_of_degrees(n: usize, degrees: usize, base: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n < base {
        if degrees == 1 {
            return 1;
        }
        return 0;
    }
    let base_number = to_base(n, base);
    let mut res = 0;
    let mut crt_digit_idx = 0;
    let mut digits_left = degrees;
    let digits_count = base_number.len();
    while crt_digit_idx < digits_count {
        let crt_digit = base_number[crt_digit_idx];
        if crt_digit > 1 {
            res += comb_n_by_k(digits_count - crt_digit_idx, digits_left);
            break;
        } else if crt_digit == 1 {
            if crt_digit_idx + 1 == digits_count {
                if digits_left == 1 {
                    res += 1;
                }
                break;
            } else {
                res += comb_n_by_k(digits_count - crt_digit_idx - 1, digits_left);
                digits_left -= 1;
                if digits_left == 0 {
                    res += 1;
                    break;
                }
            }
        } //crt_digit == 0 so skipped
        crt_digit_idx += 1;
    }
    res
}

fn main() {
    let stdin = stdin();
    let mut range_line = String::new();
    stdin.read_line(&mut range_line).unwrap();
    let mut rang_iter = range_line.split_whitespace();
    let from: usize = rang_iter.next().unwrap().parse().unwrap();
    let to: usize = rang_iter.next().unwrap().parse().unwrap();

    let mut degrees_line = String::new();
    stdin.read_line(&mut degrees_line).unwrap();
    let degrees: usize = degrees_line.trim_right().parse().unwrap();

    let mut base_line = String::new();
    stdin.read_line(&mut base_line).unwrap();
    let base: usize = base_line.trim_right().parse().unwrap();
    println!(
        "{}",
        amount_of_degrees(to, degrees, base) - amount_of_degrees(from - 1, degrees, base)
    );
}
