/**
 * Trivial string handling stuff.
 */
use std::io::stdin;
use std::io::*;

fn main() {
    let mut new_sentence: bool = true;
    let mut errs = 0;
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let mut new_word: bool = true;
        let crt_line = line.unwrap();
        for c in crt_line.chars() {
            if c.is_ascii_alphabetic() {
                if new_sentence {
                    new_sentence = false;
                    new_word = false;
                    if c.is_lowercase() {
                        errs += 1;
                    }
                } else {
                    if new_word {
                        new_word = false;
                    } else {
                        if c.is_uppercase() {
                            errs += 1;
                        }
                    }
                }
            } else if c == '.' || c == '?' || c == '!' {
                new_word = true;
                new_sentence = true;
            } else {
                new_word = true;
            }
        }
    }
    println!("{}", errs);
}
