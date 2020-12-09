//! # 203. 移除链表元素
//!
//! 难度 简单
//!
//! 删除链表中等于给定值 val 的所有节点。
//!
//! ## 示例:
//!
//! ```plain
//! 输入: 1->2->6->3->4->5->6, val = 6
//! 输出: 1->2->3->4->5
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/remove-linked-list-elements/)

use crate::util::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{ next: head, val: 0 }));
        let mut node = &mut dummy;
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            if node.as_ref().unwrap().next.as_ref().unwrap().val == val {
                let n = node.as_mut().unwrap().next.take();
                node.as_mut().unwrap().next = n.unwrap().next;
                continue;
            }
            node = &mut node.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v, t| ListNode::into_vec(Solution::remove_elements(ListNode::from_vec(v), t));
        assert_eq!(vec![1,2,3,4,5], t(vec![1,2,6,3,4,5,6], 6));
        assert_eq!(vec![0i32;0], t(vec![], 6));
    }
}
