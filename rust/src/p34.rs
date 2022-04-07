#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32) -> Result<usize, usize> {
        if nums.len() == 0 {
            return Err(0);
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if nums[l] == target {
            Ok(l)
        } else if nums[l] < target {
            Err(l + 1)
        } else {
            Err(l)
        }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let a = match Solution::binary_search(&nums, target) {
            Ok(pos) => pos as i32,
            Err(_pos) => -1,
        };

        let b = if a >= 0 {
            let c = target + 1;

            match Solution::binary_search(&nums, c) {
                Ok(pos) => (pos - 1) as i32,
                Err(pos) => (pos - 1) as i32,
            }
        } else {
            -1
        };

        vec![a, b]
    }
}
