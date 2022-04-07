#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }
        let (first, rest) = nums.split_at(1);
        let mut ret = Solution::subsets(rest.to_vec());
        for set in ret.clone() {
            let mut tmp = set;
            tmp.append(&mut vec![first[0]]);
            ret.push(tmp);
        }
        ret
    }
}
