mod leetcode_2055;

fn main() {
    println!(
        "{:?}",
        leetcode_2055::Solution::plates_between_candles(
            String::from("***|**|*****|**||**|*"),
            vec![
                vec![1, 17],
                vec![4, 5],
                vec![14, 17],
                vec![5, 11],
                vec![15, 16]
            ]
        )
    );
}
