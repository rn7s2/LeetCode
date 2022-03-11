#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 {
            return 0;
        }

        let mut product = 1;
        let mut ans = 0;
        let mut left = 0;
        let len = nums.len();
        for right in 0..len {
            product = product * nums[right];
            while left <= right && product >= k {
                product /= nums[left];
                left = left + 1;
            }
            // Here, nums[right] is always selected.
            // New subarrays are [left..right], [left + 1..right]..[right]
            ans = ans + right - left + 1;
        }

        ans as i32
    }
}
