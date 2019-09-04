const MAX_N: usize = 1000001;
fn main() {
    let sieve = primal::Sieve::new(MAX_N);
    let mut phi_by_n: [f64; MAX_N] = [1f64; MAX_N];
    phi_by_n[1] = 1.0;    
    for p in sieve.primes_from(2) {
        let ratio =(p as f64) / (p-1) as f64;
        phi_by_n[p] = ratio;
        let mut i=2;
        let mut idx = p*i;
        while idx < MAX_N {            
            phi_by_n[idx] = phi_by_n[idx] * ratio;
            i +=1;
            idx = p*i;
        }

    }
    
    let mut max_val = 0.0;
    let mut max_idx: usize = 0;    
    for i in 2..MAX_N {
        if phi_by_n[i] > max_val {
            max_val = phi_by_n[i];
            max_idx =i;
        }
    }
    println!("{}", max_idx );
}