use primal::is_prime;
use primal::Sieve;

const LIMIT: usize = 9000;

#[inline]
fn digit_count(mut d: usize) -> u32 {    
    let mut count = 1;    
    d = d / 10;
    while d != 0 {
        count +=1;
        d = d /10;
    }
    count 
}

#[inline]
fn prime_pair(p1: usize, p2: usize, sieve: &Sieve) -> bool {
    
    check_prime(p1 * 10usize.pow(digit_count(p2)) + p2, sieve)
        && check_prime(p2 * 10usize.pow(digit_count(p1)) + p1, sieve)
}

#[inline]
fn check_prime(p: usize, sieve: &Sieve) -> bool {
    if sieve.upper_bound() >= p {
        sieve.is_prime(p)
    } else {
        is_prime(p as u64)
    }
}

fn main() {
    let sieve = primal::Sieve::new(LIMIT);
    for p1 in sieve.primes_from(5) {
        let mod3 = p1 % 3;
        for p2 in sieve.primes_from(p1 + 1) {
            if  p2 % 3 == mod3 && prime_pair(p1, p2, &sieve) {
                for p3 in sieve.primes_from(p2 + 1) {
                    if p3 % 3 == mod3 && prime_pair(p1, p3, &sieve) && prime_pair(p2, p3, &sieve) {
                        for p4 in sieve.primes_from(p3 + 1) {
                            if p4 % 3 == mod3 && prime_pair(p1, p4, &sieve)
                                && prime_pair(p2, p4, &sieve)
                                && prime_pair(p3, p4, &sieve)
                            {
                                for p5 in sieve.primes_from(p4 + 1) {
                                    if p5 % 3 == mod3 && prime_pair(p1, p5, &sieve)
                                        && prime_pair(p2, p5, &sieve)
                                        && prime_pair(p3, p5, &sieve)
                                        && prime_pair(p4, p5, &sieve)
                                    {
                                        println!(
                                            "VICTORY {} {} {} {} {} {} ",
                                            p1,
                                            p2,
                                            p3,
                                            p4,
                                            p5,
                                            p1 + p2 + p3 + p4 + p5
                                        );
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }    
}
