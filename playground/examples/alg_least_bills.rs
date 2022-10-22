use std::cmp::min;

// 金额为 2, 3, 7的情况下，最小的满足total的组合数
fn least_bills_iteration(total: usize) -> usize {
    if total == 0 {
        return 0;
    }
    let mut dp = vec![0x3f3f3f; total + 1];
    dp[0] = 0;
    dp[2] = 1;
    dp[3] = 1;
    dp[7] = 1;
    for i in 3..=total {
        match i {
            i if i >= 7 => dp[i] = min(min(dp[i - 2], dp[i - 3]), dp[i - 7]) + 1,
            i if i >= 3 && i < 7 => dp[i] = min(dp[i - 2], dp[i - 3]) + 1,
            _ => dp[i] = dp[i - 2] + 1,
        }
    }
    dp[total]
}

fn least_bills_dp(total: usize, moneys: Vec<usize>) -> usize {
    if total <= 0 {
        return 0;
    }
    // why 0x3f3f3f3f: https://zhuanlan.zhihu.com/p/57512786
    let mut dp = vec![0x3f3f3f3f; total + 1];
    dp[0] = 0;
    for i in 1..=total {
        for &j in moneys.iter() {
            if i >= j {
                dp[i] = min(dp[i - j] + 1, dp[i])
            }
        }
    }
    dp[total]
}

// not accurate in some cases
fn least_bills_greedy(total: usize, moneys: Vec<usize>) -> usize {
    if total <= 0 {
        return 0;
    }
    let mut result = 0;
    let mut tmp = total;
    moneys.iter().rev().enumerate().for_each(|(i, v)| {
        if tmp <= 0 {
            return;
        }

        let cnt = tmp / v;
        tmp -= v * cnt;
        result += cnt;
    });
    if tmp != 0 {
        return 0;
    }
    result
}
fn main() {
    let bill = 102;
    let moneys = vec![2, 3, 7];
    println!(
        "least_bills_iteration for bill {}: {:?} {:?} {:?}",
        bill,
        least_bills_iteration(bill),
        least_bills_dp(bill, moneys.clone()),
        least_bills_greedy(bill, moneys.clone())
    );
}
