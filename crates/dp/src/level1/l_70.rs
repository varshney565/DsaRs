fn climb_helper(n : i32,dp : &mut Vec<i32>) -> i32 {
    if n < 0 {
        return 0;
    }

    if n == 0 {
        dp[n as usize] = 1;
        return dp[n as usize];
    }
    if dp[n as usize] != -1 {
        return dp[n as usize];
    }
    dp[n as usize] = climb_helper(n-1,dp) + climb_helper(n-2,dp);
    return dp[n as usize]
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![-1;n as usize + 1];
    climb_helper(n,&mut dp)
}