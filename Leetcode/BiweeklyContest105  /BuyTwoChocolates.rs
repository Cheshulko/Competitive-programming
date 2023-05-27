impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort();
        match money - (prices[0] + prices[1]) {
            x @ 0.. => x,
            _ => money,
        }
    }
}
