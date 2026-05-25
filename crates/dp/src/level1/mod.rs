pub fn fibonacci_recursive(num : i32) -> i32{
    if num <= 1 {
        return num;
    }
    return fibonacci_recursive(num-1) + fibonacci_recursive(num-2);
}

pub fn fibonacci_memo(num : i32,dp : &mut Vec<i32>) -> i32 {
    if num <= 1 {
        dp[num as usize] = num;
        return num;
    }
    if dp[num as usize] != -1 {
        return dp[num as usize];
    }
    dp[num as usize] = fibonacci_memo(num-1,dp) + fibonacci_memo(num-2,dp);
    dp[num as usize]
}


pub fn fibonacci_tabu(n: i32, dp: &mut Vec<i32>) -> i32 {
    for num in 0..=n {
        if num <= 1 {
            dp[num as usize] = num;
            continue;
        }
        if dp[num as usize] != -1 {
            return dp[num as usize];
        }
        dp[num as usize] = dp[(num-1) as usize] + dp[(num-2) as usize];
    }
    dp[n as usize]
}