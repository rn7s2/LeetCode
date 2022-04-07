#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let chars_p = s.as_bytes();
        let chars_q = t.as_bytes();
        let mut p: i32 = s.len() as i32 - 1;
        let mut q: i32 = t.len() as i32 - 1;
        let mut cp = 0;
        let mut cq = 0;

        while p >= 0 || q >= 0 {
            while p >= 0 {
                if chars_p[p as usize] as char == '#' {
                    cp = cp + 1;
                    p = p - 1;
                } else if cp > 0 {
                    cp = cp - 1;
                    p = p - 1;
                } else {
                    break;
                }
            }
            while q >= 0 {
                if chars_q[q as usize] as char == '#' {
                    cq = cq + 1;
                    q = q - 1;
                } else if cq > 0 {
                    cq = cq - 1;
                    q = q - 1;
                } else {
                    break;
                }
            }
            if !(p + 1 > 0 && q + 1 > 0) {
                break;
            }
            if chars_p[p as usize] != chars_q[q as usize] {
                return false;
            }
            if p >= 0 {
                p = p - 1;
            }
            if q >= 0 {
                q = q - 1;
            }
        }
        p == q && q == -1
    }
}
