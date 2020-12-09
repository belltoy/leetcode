//! # 23. 合并K个升序链表
//!
//! 难度 困难
//!
//! 给你一个链表数组，每个链表都已经按升序排列。
//!
//! 请你将所有链表合并到一个升序链表中，返回合并后的链表。
//!
//! ## 示例 1：
//!
//! ```text
//! 输入：lists = [[1,4,5],[1,3,4],[2,6]]
//! 输出：[1,1,2,3,4,4,5,6]
//! 解释：链表数组如下：
//! [
//!   1->4->5,
//!   1->3->4,
//!   2->6
//! ]
//! 将它们合并到一个有序链表中得到。
//! 1->1->2->3->4->4->5->6
//! ```
//!
//! ## 示例 2：
//!
//! ```text
//! 输入：lists = []
//! 输出：[]
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：lists = [[]]
//! 输出：[]
//! ```
//!
//! ## 提示：
//!
//! - `k` == `lists.length`
//! - `0` <= `k` <= `10^4`
//! - `0` <= `lists[i].length` <= `500`
//! - `-10^4` <= `lists[i][j]` <= `10^4`
//! - `lists[i]` 按 **升序** 排列
//! - `lists[i].length` 的总和不超过 `10^4`
//!
//! See [leetcode](https://leetcode-cn.com/problems/merge-k-sorted-lists/)

// use crate::ListNode;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use std::cmp::{Ord, Ordering, PartialEq, Reverse};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {

    /// 使用 BinaryHeap，但需要转化为小顶堆
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut priority_queue = BinaryHeap::with_capacity(lists.len());

        for node in lists {
            if let Some(node) = node {
                priority_queue.push(Reverse(node))
            }
        }

        let mut result = Box::new(ListNode::new(0));
        let mut ptr = &mut result;
        while let Some(Reverse(mut node)) = priority_queue.pop() {
            if let Some(next) = node.next.take() {
                priority_queue.push(Reverse(next));
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap()
        }

        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, ListNode};

    fn slice_to_list(a: &[i32]) -> Option<Box<ListNode>> {
        a.iter().fold(None, |mut list, &v| {
            let node = Box::new(ListNode::new(v));
            if let Some(ref mut a) = list {
                let mut last = &mut a.next;
                while let Some(ref mut node) = last {
                    last = &mut node.next;
                }
                *last = Some(node);
                list
            } else {
                Some(node)
            }
        })
    }

    fn list_to_slice(mut a: Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::from_fn(move || {
            if let Some(mut v) = a.take() {
                let item = Some(v.val);
                a = v.next.take();
                item
            } else {
                None
            }
        }).collect()
    }

    #[test]
    fn test() {
        let lists: Vec<_> = vec![vec![1,4,5], vec![1,3,4], vec![2,6]].into_iter().map(|a| slice_to_list(&a[..])).collect();
        assert_eq!(&[1i32,1,2,3,4,4,5,6][..], list_to_slice(Solution::merge_k_lists(lists)));

        let lists: Vec<_> = vec![].into_iter().map(|a: Vec<i32>| slice_to_list(&a[..])).collect();
        let expect: &[i32] = &[][..];
        assert_eq!(expect, list_to_slice(Solution::merge_k_lists(lists)));

        let lists: Vec<_> = vec![vec![]].into_iter().map(|a| slice_to_list(&a[..])).collect();
        let expect: &[i32] = &[][..];
        assert_eq!(expect, list_to_slice(Solution::merge_k_lists(lists)));
    }
}
