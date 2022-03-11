#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let l = 0 as usize;
        let r = nums.len() - 1;

        if l == r {
            return l as i32;
        }

        let m = (l + r) / 2;
        let (left, right) = nums.split_at(m + 1);
        if nums[m] < nums[m + 1] {
            (m + 1) as i32 + Solution::find_peak_element(right.to_vec())
        } else {
            Solution::find_peak_element(left.to_vec())
        }
    }
}
