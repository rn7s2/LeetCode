#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let l = 0 as usize;
        let r = nums.len() - 1;
        if l == r {
            nums[l]
        } else {
            let m = (l + r) / 2;
            let (left, right) = nums.split_at(m + 1);
            if nums[l] < nums[m] {
                let l_min = nums[l];
                let r_min = Solution::find_min(right.to_vec());
                std::cmp::min(l_min, r_min)
            } else {
                let l_min = Solution::find_min(left.to_vec());
                let r_min = nums[m + 1];
                std::cmp::min(l_min, r_min)
            }
        }
    }
}
