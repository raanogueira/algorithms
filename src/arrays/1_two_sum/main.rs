use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::two_sum(vec![1, 2, 3, 4, 5], 5))
}

struct Solution;
impl Solution {
    /**
     * Problem: https://leetcode.com/problems/two-sum/
     *
     * Rationale:
     * Brute Force: O(n^2)
     * Use a hashmap to keep track of the numbers already considered while trying to find if the difference between target and current is already in the map.
     *
     * Time: O(n)
     * Space: O(n)
     */
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_by_index: HashMap<i32, i32> = HashMap::new();
        for (i, value) in nums.iter().enumerate() {
            let diff = target - value;
            match nums_by_index.get(&diff) {
                Some(complement_index) => return vec![complement_index.clone(), i as i32],
                None => {
                    nums_by_index.insert(value.clone(), i as i32);
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::Solution;

    #[test]
    fn it_finds_two_sum_case1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn it_finds_two_sum_case2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn it_finds_two_sum_case3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
