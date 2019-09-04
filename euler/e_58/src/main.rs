fn mod_exp(mut x: u64, mut d: u64, n: u64) -> u64 {
    let mut ret = 1;
    while d != 0 {
        if d % 2 == 1 {
            ret *= x;
            if ret >= n {
                ret %= n;
            }
        }
        d /= 2;
        x *= x;
        if x >= n {
            x %= n;
        }
    }
    ret
}

pub fn is_prime(n: u64) -> bool {
    const HINT: &'static [u64] = &[2];

    const WITNESSES: &'static [(u64, &'static [u64])] =
        &[(2_046, HINT),
          (1_373_652, &[2, 3]),
          (9_080_190, &[31, 73]),
          (25_326_000, &[2, 3, 5]),
          (4_759_123_140, &[2, 7, 61]),
          (1_112_004_669_632, &[2, 13, 23, 1662803]),
          (2_152_302_898_746, &[2, 3, 5, 7, 11]),
          (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
          (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
          (0xFFFF_FFFF_FFFF_FFFF, &[2, 3, 5, 7, 11, 13, 17, 19, 23])
         ];

    if n % 2 == 0 { return n == 2 }
    if n == 1 { return false }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 { 
        d /= 2; s += 1;
    }

    let witnesses =
        WITNESSES.iter().find(|&&(hi, _)| hi >= n)
            .map(|&(_, wtnss)| wtnss).unwrap();
    'next_witness: for &a in witnesses.iter() {
        let mut power = mod_exp(a, d, n);
        if power == 1 { continue 'next_witness }

        for _r in 0..s {
            if power == n - 1 {
                continue 'next_witness
            }
            power *= power;
            if power >= n {
                power %= n;
            }
        }
        return false
    }

    true
}


fn main() {
    
    let mut total_numbers:u64 = 1;
    let mut total_primes:u64 = 0;
    let mut step = 0;    
    loop{
        step +=1;
        total_numbers+=4;
        let next_corner:u64 = (2*step+1)*(2*step+1);
        let delta:u64 = 2 * step;
        for i in 1..4 {
            let corner = next_corner - delta * i;
            if is_prime(corner){
                total_primes+=1;
            }
        }
        if total_primes * 10 < total_numbers {
            println!("{}: Numbers:{}, Primes {}, Len {}, Corner {}",step, total_numbers, total_primes, 2*step+1,next_corner);
            break;
        }        
    }            
}
