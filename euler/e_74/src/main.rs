use bit_set::BitSet;
use std::collections::HashSet;

const MAX_N: u32 = 1_000_000;
const FACT: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

#[inline]
fn next(mut n: u32) -> u32 {
    let mut res = 0;
    while n != 0 {
        res += FACT[n as usize % 10];
        n = n / 10;
    }
    res
}

fn main() {
    let mut good = BitSet::new();
    let mut bad = BitSet::new();
    let mut chains = 0;
    let mut chain  = HashSet::new();
    let mut checks = 0;
    for i in 2..MAX_N {        
        let fact_i = next(i) as usize;
        if good.contains(fact_i) {
            chains += 1;
            continue;
        } else if bad.contains(fact_i) {
            continue;
        }
        //dbg!(i);
        checks +=1;
        chain.insert(i);
        let mut next_in_chain = fact_i as u32;        
        while chain.len() <= 60  && !chain.contains(&next_in_chain)  {
            chain.insert(next_in_chain);            
            next_in_chain = next(next_in_chain);            
        }
        
        if chain.len() == 60 {
            chains += 1;
            good.insert(fact_i);
        } else {            
            bad.insert(fact_i);
        }
        chain.clear();
    }
    println!("{} {}", chains,checks);
}
