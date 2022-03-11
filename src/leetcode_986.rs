#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    // This recursive solution is not expected to run fast.
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        if first_list.len() == 0 || second_list.len() == 0 {
            return vec![];
        }

        if first_list[0][1] <= second_list[0][1] {
            let (_first, new_first) = first_list.split_first().unwrap();
            if first_list[0][1] >= second_list[0][0] {
                let elem = vec![
                    std::cmp::max(first_list[0][0], second_list[0][0]),
                    first_list[0][1],
                ];
                let mut tmp = Solution::interval_intersection(new_first.to_vec(), second_list);
                tmp.insert(0, elem);
                tmp
            } else {
                Solution::interval_intersection(new_first.to_vec(), second_list)
            }
        } else {
            let (_first, new_second) = second_list.split_first().unwrap();
            if second_list[0][1] >= first_list[0][0] {
                let elem = vec![
                    std::cmp::max(first_list[0][0], second_list[0][0]),
                    second_list[0][1],
                ];
                let mut tmp = Solution::interval_intersection(first_list, new_second.to_vec());
                tmp.insert(0, elem);
                tmp
            } else {
                Solution::interval_intersection(first_list, new_second.to_vec())
            }
        }
    }
}
