const MAX_N: usize = 10_000_000_001;

fn main() {
    let sieve = primal_sieve::Sieve::new(MAX_N);
    let mut total = MAX_N -1;
    for p in sieve.primes_from(2) {
        if p >= MAX_N {
            break;
        }
        let ratio = MAX_N /p;
        if p > ratio {
            total -=ratio;
        }else{
            total -=p;
        }        
    }
    println!("{}", total );    
     
}
