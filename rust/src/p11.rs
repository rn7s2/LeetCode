#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0 as usize;
        let mut r = height.len() - 1;
        let mut max = -1;

        while l < r {
            max = std::cmp::max(max, (r - l) as i32 * std::cmp::min(height[l], height[r]));
            if height[l] < height[r] {
                l = l + 1;
            } else {
                r = r - 1;
            }
        }

        max
    }
}
