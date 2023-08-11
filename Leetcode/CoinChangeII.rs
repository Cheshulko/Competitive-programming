struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;

        let mut coins_with_0 = vec![0];
        coins_with_0.extend(coins.into_iter().map(|x| x as usize).collect::<Vec<_>>());

        let mut cnt = vec![0; 5000 + 1];
        cnt[0] = 1;

        for coin in &coins_with_0[1..] {
            for c in 0..=(5000 - *coin) {
                cnt[*coin + c] += cnt[c];
            }
        }

        cnt[amount]
    }
}
