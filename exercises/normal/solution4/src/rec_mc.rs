pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }
    let coins = [1, 2, 5, 10, 20, 30, 50, 100];
    let max_val = u32::MAX;
    let mut dp = vec![max_val; (amount + 1) as usize];
    dp[0] = 0;
    
    for i in 1..=amount {
        for &c in coins.iter() {
            if c <= i {
                let prev = i - c;
                if dp[prev as usize] != max_val {
                    dp[i as usize] = dp[i as usize].min(dp[prev as usize] + 1);
                }
            }
        }
    }
    
    dp[amount as usize]
}