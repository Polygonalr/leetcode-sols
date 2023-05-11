impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut low = prices[0];
        let mut high = prices[0];
        let mut best = 0;
        for price in prices {
            if price < low {
                high = price;
                low = price;
            } else if price > high {
                high = price;
                best = std::cmp::max(best, high - low);
            }
        }
        best
    }
}
