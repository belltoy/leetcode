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
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut rest = &None;
        let mut carry = 0;
        let mut sum_tail = &mut sum;
        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                rest = l2;
                break;
            }
            if l2.is_none() {
                rest = l1;
                break;
            }

            let x = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val + carry;
            let (x, c) = if x >= 10 {
                (x % 10, 1)
            } else {
                (x, 0)
            };
            carry = c;
            sum_tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(x)));
            sum_tail = &mut sum_tail.as_mut().unwrap().next;

            l1 = &l1.as_ref().unwrap().next;
            l2 = &l2.as_ref().unwrap().next;
        }

        while rest.is_some() {
            let x = rest.as_ref().unwrap().val + carry;
            let (x, c) = if x >= 10 {
                (x % 10, 1)
            } else {
                (x, 0)
            };
            carry = c;
            sum_tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(x)));
            sum_tail = &mut sum_tail.as_mut().unwrap().next;
            rest = &rest.as_ref().unwrap().next;
        }

        if carry > 0 {
            sum_tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
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
