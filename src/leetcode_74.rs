#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn row_smaller(row: &Vec<i32>, target: i32) -> bool {
        return row[row.len() - 1] < target;
    }

    fn row_contains(row: &Vec<i32>, target: i32) -> bool {
        return row[0] <= target && target <= row[row.len() - 1];
    }

    fn binary_search(matrix: &Vec<Vec<i32>>, target: i32) -> Result<usize, usize> {
        if matrix.len() == 0 {
            return Err(0);
        }

        let mut l = 0;
        let mut r = matrix.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if Solution::row_smaller(&matrix[m], target) {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if Solution::row_contains(&matrix[l], target) {
            Ok(l)
        } else if Solution::row_smaller(&matrix[l], target) {
            Err(l + 1)
        } else {
            Err(l)
        }
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let pos = match Solution::binary_search(&matrix, target) {
            Ok(pos) => pos as i32,
            Err(_pos) => -1,
        };

        if pos < 0 {
            return false;
        } else {
            return match matrix[pos as usize].binary_search(&target) {
                Ok(_pos) => true,
                Err(_pos) => false,
            };
        }
    }
}
