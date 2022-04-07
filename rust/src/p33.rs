#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let l = 0 as usize;
        let r = nums.len() - 1;

        if l == r {
            return if nums[l] == target { 0 } else { -1 };
        } else {
            let m = (l + r) / 2;
            let (left, right) = nums.split_at(m + 1);

            if nums[l] < nums[m] {
                let pos = Solution::search(right.to_vec(), target);
                if pos >= 0 {
                    return m as i32 + pos + 1;
                } else {
                    return match left.to_vec().binary_search(&target) {
                        Ok(pos) => pos as i32,
                        Err(_pos) => -1,
                    };
                }
            } else {
                let pos = Solution::search(left.to_vec(), target);
                if pos >= 0 {
                    return pos;
                } else {
                    return match right.to_vec().binary_search(&target) {
                        Ok(pos) => (m + pos + 1) as i32,
                        Err(_pos) => -1,
                    };
                }
            }
        }
    }
}
