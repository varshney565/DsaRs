

pub fn maze_path_jump(n : i32,m : i32) -> i32 {
    let dir  = vec![vec![-1,0],vec![-1,-1],vec![0,-1]];
    let mut dp = vec![vec![-1;m as usize];n as usize];
    maze_path_jump_helper(n, m, &dir, &mut dp)
}

pub fn maze_path_jump_helper(n : i32,m : i32,dir : &Vec<Vec<i32>>,dp : &mut Vec<Vec<i32>>) -> i32 {
     if n == 0 && m == 0 {
        dp[n as usize][m as usize] = 1;
        return dp[n as usize][m as usize];
    }
    if dp[n as usize][m as usize] != -1 {
        return dp[n as usize][m as usize];
    }
    let mut ans = 0;
    for di in dir {
        let mut newx = n + di[0];
        let mut newy = m + di[1];
        while newx >= 0 && newy >= 0 {
            ans = ans + maze_path_jump_helper(newx, newy,dir,dp);
            newx += di[0];
            newy += di[1];
        }
    }
    dp[n as usize][m as usize] = ans;
    dp[n as usize][m as usize]
}


