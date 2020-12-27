//! # 2. 两数相加
//!
//! 难度 中等
//!
//! 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
//!
//! 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
//!
//! 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//!
//! ## 示例：
//!
//! ```text
//! 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
//! 输出：7 -> 0 -> 8
//! 原因：342 + 465 = 807
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/add-two-numbers/)
use crate::ListNode;

/// 这一题与 [sum_lists](super::sum_lists) 基本上一样，除了进阶。
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut t = (l1, l2, 0, 0); // (list1, list2, sum, carry)
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry) if list.val + carry >= 10 => {
                    (list.next, None, list.val + carry - 10, 1)
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    (list.next, None, list.val + carry, 0)
                }
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                    (l1.next, l2.next, l1.val + l2.val + carry - 10, 1)
                }
                (Some(l1), Some(l2), _, carry) => {
                    (l1.next, l2.next, l1.val + l2.val + carry, 0)
                }
            };

            *tail = Some(Box::new(ListNode::new(t.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![], (list![], list![])),
            (vec![7,0,8], (list![7,0,8], list![])),
            (vec![7,0,8], (list![], list![7,0,8])),
            (vec![7,0,8], (list![2,4,3], list![5,6,4])),
            (vec![2,1,9], (list![7,1,6], list![5,9,2])),
            (vec![2,1,1,1], (list![7,1,6], list![5,9,4])),
        ];

        for (expect, (l1, l2)) in cases {
            assert_eq!(expect, ListNode::into_vec(Solution::add_two_numbers(l1, l2)));
        }
    }
}
