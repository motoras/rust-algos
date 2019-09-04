
fn prime_sum(n:usize)->usize{    
    let sieve = primal::Sieve::new(n+1);
    let prime_count = sieve.prime_pi(n+1);
    let mut dp = [[0;1000];1000];
    for i in 0..=prime_count {
        dp[i][0] =1;
    }         
    for i in 1..=prime_count {        
        for j in 1..=n{
            if i> 1 && sieve.nth_prime(i-1) <= j{
                dp[i][j] = dp[i-1][j] + dp[i][j-sieve.nth_prime(i-1)];
            }else{
                dp[i][j] =dp[i-1][j];
            }                
        }        
    }
    dp[prime_count][n]
}

fn main() {    
    for i in 1..100 {
        println!("{}->{}",i,prime_sum(i));
    }    
    
}