#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut a = nums.clone();
        a.sort();

        let mut ans = Vec::new();
        let mut last_first_num = -100000;
        for i in 0..a.len() - 2 {
            if last_first_num == a[i] {
                continue;
            }
            last_first_num = a[i];

            let mut last_second_num = -100000;
            let mut k = a.len() - 1;
            for j in i + 1..a.len() - 1 {
                if j >= k {
                    break;
                }
                if last_second_num == a[j] {
                    continue;
                }
                last_second_num = a[j];
                while a[i] + a[j] + a[k] > 0 && j < k - 1 {
                    k = k - 1;
                }
                if a[i] + a[j] + a[k] == 0 {
                    ans.push(vec![a[i], a[j], a[k]]);
                }
            }
        }

        ans
    }
}
