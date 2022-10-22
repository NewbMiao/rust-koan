use std::cmp::min;
// https://alchemist-al.com/algorithms/edit-distance
fn levenshtein(m1: &str, m2: &str) -> Option<usize> {
    let l1: usize = m1.len();
    let l2: usize = m2.len();
    if l1 == 0 || l2 == 0 {
        return None;
    }
    let mut dp: Vec<Vec<usize>> = (1..)
        .take(l1 + 1)
        .map(|_| (1..).take(l2 + 1).map(|_| 0).collect())
        .collect();
    dp[0].iter_mut().enumerate().for_each(|(i, v)| {
        *v = i;
    });
    dp.iter_mut().enumerate().for_each(|(i, v)| v[0] = i);
    // d[i+1, j+1] = min(d[i, j+1] + 1, d[i+1, j] + 1, d[i, j] + r(i, j)
    for (i, char1) in m1.chars().enumerate() {
        for (j, char2) in m2.chars().enumerate() {
            let mut r = 0;
            if char1 != char2 {
                r = 1;
            }
            dp[i + 1][j + 1] = min(min(dp[i + 1][j] + 1, dp[i][j + 1] + 1), dp[i][j] + r);
        }
    }
    println!("{:?}", dp);

    Some(dp[l1][l2])
}

fn levenshtein_advanced(m1: &str, m2: &str) -> Option<usize> {
    let l1: usize = m1.len();
    let l2: usize = m2.len();
    if l1 == 0 || l2 == 0 {
        return None;
    }
    if l1 < l2 {
        return levenshtein_advanced(m2, m1);
    }
    let mut dp: Vec<usize> = (0..).take(l2 + 1).collect();
    // d[i+1, j+1] = min(d[i, j+1] + 1, d[i+1, j] + 1, d[i, j] + r(i, j)
    let mut pre;
    let mut tmp;
    for (i, char1) in m1.chars().enumerate() {
        pre = dp[0];
        // prepared for first column of next row
        dp[0] = i + 1;
        for (j, char2) in m2.chars().enumerate() {
            tmp = dp[j + 1];
            let mut r = 0;
            if char1 != char2 {
                r = 1;
            }
            dp[j + 1] = min(
                min(
                    dp[j] + 1, // delete
                    tmp + 1,   // insert
                ),
                pre + r, // match or update
            );
            pre = tmp;
        }
    }
    println!("{:?}", dp);
    Some(dp[l2])
}

fn main() {
    let m = levenshtein("dog", "doge");
    println!("---{:?}", m);

    let m = levenshtein_advanced("dog", "doge");
    println!("==={:?}", m);
}
