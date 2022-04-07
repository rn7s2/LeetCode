#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let node_head = match head {
            Some(v) => v,
            None => return None,
        };
        let first_num = node_head.clone().val;
        let mut first_num_cnt = 0;

        let mut iter_head = node_head.clone();
        loop {
            if iter_head.val != first_num {
                break;
            }
            first_num_cnt = first_num_cnt + 1;
            iter_head = match iter_head.next {
                Some(v) => v,
                None => break,
            };
        }

        if iter_head.val == first_num && iter_head.next == None {
            return if first_num_cnt == 1 {
                Some(iter_head)
            } else {
                None
            };
        }

        if first_num_cnt > 1 {
            Solution::delete_duplicates(Some(iter_head))
        } else {
            Some(Box::new(ListNode {
                next: Solution::delete_duplicates(Some(iter_head)),
                val: first_num,
            }))
        }
    }
}
