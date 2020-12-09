//! # 61. 旋转链表
//!
//! 难度 中等
//!
//! 给定一个链表，旋转链表，将链表每个节点向右移动 k 个位置，其中 k 是非负数。
//!
//! ## 示例 1:
//!
//! ```plain
//! 输入: 1->2->3->4->5->NULL, k = 2
//! 输出: 4->5->1->2->3->NULL
//! 解释:
//! 向右旋转 1 步: 5->1->2->3->4->NULL
//! 向右旋转 2 步: 4->5->1->2->3->NULL
//! ```
//!
//! ## 示例 2:
//!
//! ```plain
//! 输入: 0->1->2->NULL, k = 4
//! 输出: 2->0->1->NULL
//! 解释:
//! 向右旋转 1 步: 2->0->1->NULL
//! 向右旋转 2 步: 1->2->0->NULL
//! 向右旋转 3 步: 0->1->2->NULL
//! 向右旋转 4 步: 2->0->1->NULL
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/rotate-list/)

use crate::util::ListNode;

pub struct Solution;

impl Solution {

    /// 双指针，两个 mut 所以需要用到 unsafe
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        debug_assert!(k >= 0);

        if head.is_none() {
            return None;
        }

        if k == 0 {
            return head;
        }

        let mut len = 0;
        let mut ptr = &head;
        while let Some(ref node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        let step = k % len;
        if step == 0 {
            return head;
        }


        let mut fast: *mut _ = &mut head;
        let mut slow: *mut _ = fast;
        unsafe {
            // fast pointer go first
            for _ in 0..step {
                fast = &mut (*fast).as_mut().unwrap().next;
            }

            // move fast & slow together until fast reach the end
            while (*fast).is_some() && (*fast).as_ref().unwrap().next.is_some() {
                fast = &mut (*fast).as_mut().unwrap().next;
                slow = &mut (*slow).as_mut().unwrap().next;
            }

            (*fast).as_mut().unwrap().next = head;
            let new_head = (*slow).as_mut().unwrap().next.take();
            new_head
        }
    }

    pub fn rotate_right_unsafe(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        debug_assert!(k >= 0);

        if head.is_none() {
            return None;
        }

        if k == 0 {
            return head;
        }

        let mut len = 0;
        let mut tail = &head;
        while let Some(ref node) = tail {
            len += 1;
            if node.next.is_none() {
                break;
            }
            tail = &node.next;
        }

        let step = k % len;
        if step == 0 {
            return head;
        }

        let mut new_head = &head;
        for _ in 0..step {
            new_head = &new_head.as_ref().unwrap().next;
        }

        unsafe {
            let tail = tail as *const _ as *mut Option<Box<ListNode>>;
            let new_head = new_head as *const _ as *mut Option<Box<ListNode>>;
            (*tail).as_mut().unwrap().next = head;
            (*new_head).as_mut().unwrap().next.take()
        }
    }

    /// 单指针（引用），安全版本
    pub fn rotate_right_safe(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        debug_assert!(k >= 0);

        if head.is_none() {
            return None;
        }

        if k == 0 {
            return head;
        }

        let mut len = 0;
        let mut ptr = &head;
        while let Some(ref node) = ptr {
            len += 1;
            ptr = &node.next;
        }

        let step = k % len;

        if step == 0 || len == 1 {
            return head;
        }

        let mut ptr = &mut head;
        for _ in 1..(len - step) {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        let mut new_head = ptr.as_mut().unwrap().next.take();
        let mut ptr = &mut new_head;
        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        ptr.as_mut().unwrap().next = head;

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v, k| ListNode::into_vec(Solution::rotate_right(ListNode::from_vec(v), k));
        assert_eq!(vec![4,5,1,2,3], t(vec![1,2,3,4,5], 2));
        assert_eq!(vec![1,2,3,4,5], t(vec![1,2,3,4,5], 0));
        assert_eq!(vec![0i32;0], t(vec![], 2));
        assert_eq!(vec![0i32;0], t(vec![], 0));
    }
}
