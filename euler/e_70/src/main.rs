const MAX_N: usize = 10_000_000;

const ZERO: [i64; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[inline]
fn same_digits(mut n1: u32, mut n2: u32) -> bool {
    let mut digits = [0i64; 10];
    while n1 > 0 {
        digits[(n1 % 10) as usize] += 1;
        n1 = n1 / 10;
    }
    while n2 > 0 {
        digits[(n2 % 10) as usize] -= 1;
        n2 = n2 / 10;
    }
    digits == ZERO
}

fn brute_force() {
    let sieve = primal::Sieve::new(MAX_N);
    //dbg!("Sieve complete");
    let mut phi_rad_n: Vec<u32> = vec![1; MAX_N];
    let mut rad_n: Vec<u32> = vec![1; MAX_N];
    phi_rad_n[1] = 1;
    rad_n[1] = 1;
    //dbg!("Phi Sieve building");
    for p in sieve.primes_from(2) {
        let px = p as u32;
        phi_rad_n[p] = px - 1;
        rad_n[p] = px;
        let mut i = 2;
        let max_i = MAX_N / p;
        let eq_ok = MAX_N % p != 0;
        while max_i > i || (eq_ok && max_i == i) {
            let idx = p * i;
            phi_rad_n[idx] *= px - 1;
            rad_n[idx] *= px;
            i += 1;
        }
    }
    //dbg!("Searching...");
    let mut max_val = 0.0;
    let mut max_idx = 0;
    for i in 2..MAX_N {
        let ratio = phi_rad_n[i] as f32 / rad_n[i] as f32;
        if ratio >= max_val {
            let phi = i as u32 / rad_n[i] * phi_rad_n[i];
            if same_digits(i as u32, phi) {
                max_val = ratio;
                max_idx = i;
                //println!("{}->{}, {} {} {} ", i, phi, ratio, phi_rad_n[i], rad_n[i]);
            }
        }
    }
    let phi = max_idx as u32 / rad_n[max_idx as usize] * phi_rad_n[max_idx as usize];
    let ratio = phi_rad_n[max_idx] as f32 / rad_n[max_idx] as f32;
    println!("{} {} {}", max_idx, phi, ratio);
}

#[inline]
fn smart() {
    let sieve = primal::Sieve::new(MAX_N/2+1);
    let mut max_val = 0.0;
    let mut max_idx = 0;
    let mut max_phi = 0;
    for p in sieve.primes_from(2) {        
        for q in sieve.primes_from(p + 1) {
            let phi = (p - 1) * (q - 1);
            let n = p * q;
            if n < MAX_N {
                let ratio = phi as f32 / n as f32;
                if ratio > max_val && same_digits(n as u32, phi as u32) {
                    max_val = ratio;
                    max_idx = n;
                    max_phi = phi;
                }
            }else{
                break;
            }
        }
    }
    println!("{} {} {}", max_idx, max_phi, max_val);
}

#[inline]
fn main() {
    brute_force();
    smart();    
}
