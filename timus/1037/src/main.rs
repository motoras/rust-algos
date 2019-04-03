/**
 * Pretty much straigth forward solution. It's just a matter of chosing the 
 * right data structures. I uised a bit set to find fast the next available 
 * memory location, and to check if a location is used or not. Than store for ecah location its 
 * expiration time, and sort them by expiration in a sorted set.
 */
use std::collections::BTreeSet;
use std::collections::HashMap;

struct BitSet {
    bits: [u64; 500],
}

impl BitSet {
    #[inline]
    fn new() -> Self {
        BitSet {
            bits: [u64::max_value(); 500],
        }
    }

    #[inline]
    fn is_set(&self, bit_no: usize) -> bool {
        let offset = bit_no / 64;
        let pos = bit_no % 64;
        self.bits[offset as usize] & (1u64 << pos) != 0
    }

    #[inline]
    fn set(&mut self, bit_no: usize) {
        let offset = bit_no / 64;
        let pos = bit_no % 64;
        self.bits[offset as usize] |= 1u64 << pos;
    }

    #[inline]
    fn clear(&mut self, bit_no: usize) {
        let offset = bit_no / 64;
        let pos = bit_no % 64;
        self.bits[offset as usize] &= !(1u64 << pos);
    }

    #[inline]
    fn first_bit_set(&self) -> usize {
        for i in 0..500 {
            let crt = self.bits[i];
            if crt != 0 {
                return i * 64 + 63 - crt.leading_zeros() as usize;
            }
        }
        usize::max_value()
    }
}

use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let ten_mins = 60 * 10;
    let mut bit_set = BitSet::new();
    let mut index_by_id = HashMap::<usize, (usize, usize)>::new();
    let mut index_by_expiration = BTreeSet::<(usize, usize)>::new();
    let reader = BufReader::new(stdin());
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    for line in lines {
        let splits: Vec<&str> = line.split(" ").collect();
        let time: usize = splits[0].parse().unwrap();
        let left = index_by_expiration.split_off(&(time + 1, 0));
        index_by_expiration
            .iter()
            .for_each(|entry| bit_set.set(entry.1));
        index_by_expiration.clear();
        index_by_expiration = left;
        if splits.len() == 2 {
            let section = bit_set.first_bit_set();
            bit_set.clear(section);
            let exp = (time + ten_mins, section);
            index_by_id.insert(section, exp);
            index_by_expiration.insert(exp);
            println!("{}", (section / 64) * 64 + 63 - section % 64 + 1);
        } else {
            let block_number_raw: usize = splits[2].parse::<usize>().unwrap() - 1;
            let block_number = (block_number_raw / 64) * 64 + 63 - block_number_raw % 64;
            if bit_set.is_set(block_number) {
                println!("-");
            } else {
                index_by_expiration
                    .take(&index_by_id[&block_number])
                    .unwrap();
                let new_exp = (time + ten_mins, block_number);
                index_by_id.insert(block_number, new_exp);
                index_by_expiration.insert(new_exp);
                println!("+");
            }
        }
    }
}
