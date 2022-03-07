mod leetcode_74;

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    let target = 23;

    println!("{}", leetcode_74::Solution::search_matrix(matrix, target));
}
