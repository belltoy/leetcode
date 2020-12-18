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

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum = Some(Box::new(ListNode::new(0)));
        let mut sum_tail = sum.as_mut();
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut carry = 0;
        loop {
            let s = match (l1, l2, carry) {
                (None, None, 0) => break,
                (None, None, 1) => 1,
                (Some(n1), Some(n2), carry) => n1.val + n2.val + carry,
                (Some(n), None, carry) | (None, Some(n), carry) => n.val + carry,
                _ => unreachable!(),
            };

            let s = if s >= 10 {
                carry = 1;
                s - 10
            } else {
                carry = 0;
                s
            };

            sum_tail = sum_tail.and_then(|tail| {
                tail.next = Some(Box::new(ListNode::new(s)));
                tail.next.as_mut()
            });

            l1 = l1.and_then(|n| n.next.as_ref());
            l2 = l2.and_then(|n| n.next.as_ref());
        }
        sum.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v1, v2| ListNode::into_vec(Solution::add_two_numbers(ListNode::from_vec(v1), ListNode::from_vec(v2)));
        assert_eq!(vec![2,1,9], t(vec![7,1,6], vec![5,9,2]));
        assert_eq!(vec![2,1,1,1], t(vec![7,1,6], vec![5,9,4]));
    }
}
