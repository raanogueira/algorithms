fn main() {
    println!(
        "Max Profit: {:?}, Expected: {}",
        Solution::max_profit(vec![5, 2, 3, 4, 5]),
        3
    )
}

struct Solution;
impl Solution {
    /**
     * Problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
     *
     * Rationale:
     * Brute Force: O(n^2)
     * Use dynamyc programming (sliding window) to find the local min price. lock it and try to find the max profit in this window.
     * The min price is updated whenever we see the lowest price so far.
     *
     * This guarantees that you always use the lowest price seen in the current window while looking for the max profit.
     *
     * Time: O(n)
     * Space: O(2)
     */
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];

        for i in 1..prices.len() {
            let sell_price = prices[i] - min_price;
            if sell_price > max_profit {
                max_profit = sell_price;
            }

            if prices[i] < min_price {
                min_price = prices[i]
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::Solution;

    #[test]
    fn it_finds_best_time_to_buy_sell_case1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn it_finds_best_time_to_buy_sell_case2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }

    #[test]
    fn it_finds_best_time_to_buy_sell_case3() {
        assert_eq!(Solution::max_profit(vec![1, 3, 0, 4]), 4)
    }
}
