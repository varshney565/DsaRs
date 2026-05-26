pub fn helper(cost : &Vec<i32>,ind : i32,dp : &mut Vec<i32>) -> i32 {
    if ind <= -1 {
        dp[(ind + 2) as usize] = 0;
        return dp[(ind + 2) as usize];
    }
    if dp[(ind + 2) as usize] != -1 {
        return dp[(ind + 2) as usize];
    }
    let ans = (if ind == cost.len() as i32 { 0 } else { cost[ind as usize] }) + std::cmp::min(helper(cost,ind-1,dp),helper(cost,ind-2,dp));
    dp[(ind+2) as usize] = ans;
    return dp[(ind + 2) as usize];
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![-1;cost.len() + 3];
    helper(&cost,cost.len() as i32,&mut dp)
}