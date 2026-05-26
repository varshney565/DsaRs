//leetcode 62


fn maze_path_helper(n : i32,m : i32,dir : &Vec<Vec<i32>>,dp : &mut Vec<Vec<i32>>) -> i32 {
    if n == 0 && m == 0 {
        dp[n as usize][m as usize] = 1;
        return dp[n as usize][m as usize];
    }
    if dp[n as usize][m as usize] != -1 {
        return dp[n as usize][m as usize];
    }
    let mut ans = 0;
    for di in dir {
        let newx = n + di[0];
        let newy = m + di[1];
        if newx < 0 || newy < 0 || newx >= dp.len() as i32 || newy >= dp[0].len() as i32 {
            continue;
        }
        ans = ans + maze_path_helper(newx, newy,dir,dp);
    }
    dp[n as usize][m as usize] = ans;
    dp[n as usize][m as usize]
}

pub fn maze_path(n : i32,m : i32) -> i32 {
    let dir  = vec![vec![-1,0],vec![-1,-1],vec![0,-1]];
    let mut dp = vec![vec![-1;m as usize];n as usize];
    maze_path_helper(n-1, m-1, &dir, &mut dp)
}