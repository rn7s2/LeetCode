mod leetcode_844;

fn main() {
    println!(
        "{:?}",
        leetcode_844::Solution::backspace_compare(
            String::from("bxj##tw"),
            String::from("bxo#j##tw")
        )
    );
}
