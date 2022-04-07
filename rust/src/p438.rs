#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    fn char_to_index(ch: u8) -> usize {
        ch as usize - 'a' as usize
    }

    fn is_same(cnt: &[i32; 26], str: &[u8]) -> bool {
        let mut tmp = [0; 26];
        for i in 0..str.len() {
            let pos = Solution::char_to_index(str[i]);
            tmp[pos] = tmp[pos] + 1;
        }
        for i in 0 as usize..26 {
            if cnt[i] != tmp[i] {
                return false;
            }
        }
        true
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let str = s.as_bytes();
        let mut cnt = [0; 26];
        let mut ans = Vec::new();

        for i in 0..p.len() {
            let pos = Solution::char_to_index(str[i]);
            cnt[pos] = cnt[pos] + 1;
        }
        if Solution::is_same(&cnt, p.as_bytes()) {
            ans.push(0);
        }

        for h in 1..s.len() - p.len() + 1 {
            let pos_0 = Solution::char_to_index(str[h - 1]);
            let pos_1 = Solution::char_to_index(str[h + p.len() - 1]);
            cnt[pos_0] = cnt[pos_0] - 1;
            cnt[pos_1] = cnt[pos_1] + 1;
            if Solution::is_same(&cnt, p.as_bytes()) {
                ans.push(h as i32);
            }
        }

        ans
    }
}
