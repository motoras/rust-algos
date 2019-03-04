use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

const MAX_WORD_SIZE: usize = 50;
const IMPOSSIBLE: u8 = std::u8::MAX;
const NO_SOLUTION: &str = "No solution.";

#[inline]
fn convert(word: &str) -> String {
    let mut result = String::new();
    for c in word.chars() {
        let conver_ch = match c {
            'i' | 'j' => '1',
            'a' | 'b' | 'c' => '2',
            'd' | 'e' | 'f' => '3',
            'g' | 'h' => '4',
            'k' | 'l' => '5',
            'm' | 'n' => '6',
            'p' | 'r' | 's' => '7',
            't' | 'u' | 'v' => '8',
            'w' | 'x' | 'y' => '9',
            _ => '0',
        };
        result.push(conver_ch);
    }
    result
}

#[inline]
fn find_words_dynamic(phone_number: &str, dictionary: &HashMap<String, String>) -> String {
    let no_len = phone_number.len();
    let mut sol_len = [IMPOSSIBLE; 101];
    sol_len[0] = 0;
    let mut sol_txt = HashMap::<usize, String>::with_capacity(256);
    for i in 0..no_len {
        if sol_len[i] != IMPOSSIBLE {
            let from = i;
            let to = min(no_len, i + MAX_WORD_SIZE) + 1;
            for j in from..to {
                let candidate = &phone_number[i..j];
                let word = dictionary.get(candidate);
                if word.is_some() {
                    if sol_len[j] > 1 + sol_len[i] {
                        sol_len[j] = 1 + sol_len[i];
                        if sol_len[i] > 0 {
                            let crt_sol = sol_txt.get(&i).unwrap().clone();
                            sol_txt.insert(j, format!("{} {}", crt_sol, word.unwrap()));
                        } else {
                            sol_txt.insert(j, word.unwrap().to_string());
                        }
                    }
                }
            }
        }
    }
    if sol_len[no_len] != IMPOSSIBLE {
        sol_txt.get(&no_len).unwrap().to_string()
    } else {
        String::from(NO_SOLUTION)
    }
}

#[inline]
fn solve_dynamic(input: &mut Read, output: &mut Write) {
    let mut reader = BufReader::new(input);

    loop {
        let mut ph_line = String::new();
        reader.read_line(&mut ph_line).unwrap();
        let phone_number = ph_line.trim_right();
        if phone_number == "-1" {
            break;
        }
        let mut wc_line = String::new();
        reader.read_line(&mut wc_line).unwrap();
        let words_count: usize = wc_line.trim_right().parse().unwrap();
        if words_count == 0 {
            writeln!(output, "No solution.").unwrap();
            continue;
        }
        let mut dictionary = HashMap::<String, String>::with_capacity(words_count * 2);
        for _i in 0..words_count {
            let mut word_line = String::new();
            reader.read_line(&mut word_line).unwrap();
            let word = String::from(word_line.trim_right());
            dictionary.insert(convert(&word), word);
        }
        writeln!(output, "{}", find_words_dynamic(phone_number, &dictionary)).unwrap();
    }
}

use std::io::stdin;
use std::io::stdout;
fn main() {
    solve_dynamic(&mut stdin(), &mut stdout());
}
