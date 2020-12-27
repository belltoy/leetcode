//! # 面试题 02.05. 链表求和
//!
//! 难度 中等
//!
//! 给定两个用链表表示的整数，每个节点包含一个数位。
//!
//! 这些数位是反向存放的，也就是个位排在链表首部。
//!
//! 编写函数对这两个整数求和，并用链表形式返回结果。
//!
//!
//!
//! ## 示例：
//!
//! ```text
//! 输入：(7 -> 1 -> 6) + (5 -> 9 -> 2)，即617 + 295
//! 输出：2 -> 1 -> 9，即912
//! ```
//!
//! 进阶：思考一下，假设这些数位是正向存放的，又该如何解决呢?
//!
//! ## 示例：
//!
//! ```text
//! 输入：(6 -> 1 -> 7) + (2 -> 9 -> 5)，即617 + 295
//! 输出：9 -> 1 -> 2，即912
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/sum-lists-lcci/)
//!
use crate::ListNode;

/// 这一题与 [add_two_numbers](super::add_two_numbers) 基本上一样，增加了进阶。
///
/// 进阶的问题，先反转链表，再做同样的操作。
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

    #[test]
    fn test() {
        let cases = vec![
            (vec![], (vec![], vec![])),
            (vec![7,0,8], (vec![7,0,8], vec![])),
            (vec![7,0,8], (vec![], vec![7,0,8])),
            (vec![7,0,8], (vec![2,4,3], vec![5,6,4])),
            (vec![2,1,9], (vec![7,1,6], vec![5,9,2])),
            (vec![2,1,1,1], (vec![7,1,6], vec![5,9,4])),
        ];

        for (expect, (l1, l2)) in cases {
            assert_eq!(expect, ListNode::into_vec(Solution::add_two_numbers(ListNode::from_vec(l1), ListNode::from_vec(l2))));
        }
    }
}
