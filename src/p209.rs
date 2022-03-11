#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let len = nums.len();
        let mut l = 0;
        let mut r = 0;
        let mut sum = nums[0];
        let mut ans;

        while sum < target {
            r = r + 1;
            if r == len {
                return 0;
            }
            sum = sum + nums[r];
        }
        ans = r - l + 1;

        sum = sum - nums[r];
        for r in r..len {
            sum = sum + nums[r];
            while l < r && sum - nums[l] >= target {
                sum = sum - nums[l];
                l = l + 1;
            }
            ans = ans.min(r - l + 1);
        }

        ans as i32
    }
}
