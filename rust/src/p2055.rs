#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut candles = vec![0; s.len()];
        let mut leftmost_candle = vec![0; s.len()];
        let mut rightmost_candle = vec![0; s.len()];
        let chars = s.as_bytes();

        let mut is_candle = chars[0] as char == '|';
        candles[0] = is_candle as i32;
        leftmost_candle[0] = if is_candle { 0 } else { -1 };
        for i in 1..s.len() {
            is_candle = chars[i] as char == '|';
            candles[i] = candles[i - 1] + !is_candle as i32;
            leftmost_candle[i] = if is_candle {
                i as i32
            } else {
                leftmost_candle[i - 1] as i32
            }
        }

        is_candle = chars[s.len() - 1] as char == '|';
        rightmost_candle[s.len() - 1] = if is_candle { (s.len() - 1) as i32 } else { -1 };
        for i in (0..s.len() - 1).rev() {
            rightmost_candle[i] = if chars[i] as char == '|' {
                i as i32
            } else {
                rightmost_candle[i + 1] as i32
            }
        }

        let mut ans = Vec::new();
        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;
            let l_pos = rightmost_candle[l];
            let r_pos = leftmost_candle[r];

            let res = if l_pos < 0 || r_pos < 0 || l_pos >= r_pos {
                0
            } else {
                candles[r_pos as usize] - candles[if l_pos == 0 { 0 } else { l_pos - 1 } as usize]
            };

            ans.push(res);
        }

        ans
    }
}
