fn coin_val(i:usize)->usize{
    i
}


const AMOUNT:usize = 100;
const COINS:usize = 100;

fn main() {    
    
    let mut dp = [[0;COINS+1];AMOUNT+1];
    for i in 0..=COINS {
        dp[i][0] =1;
    }
    for i in 1..=COINS {        
        for j in 1..=AMOUNT{
            if coin_val(i-1) <= j{
                dp[i][j] = dp[i-1][j] + dp[i][j- coin_val(i-1)];
            }else{
                dp[i][j] =dp[i-1][j];
            }    
        }
    }
    println!("{}",dp[COINS][AMOUNT]);
}
