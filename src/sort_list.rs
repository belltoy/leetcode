//! # 21. 合并两个有序链表
//! 难度 简单
//!
//! 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//!
//! ## 示例：
//!
//! ```plain
//! 输入：1->2->4, 1->3->4
//! 输出：1->1->2->3->4->4
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/merge-two-sorted-lists/)


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;
type List = Option<Box<ListNode>>;
impl Solution {

    /// Bottom Up Merge Sort
    ///
    /// ## FIXME
    /// CANNOT use a raw pointer after value moved
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut length = 0;
        let mut node = &head;
        while node.is_some() {
            length += 1;
            node = &node.as_ref().unwrap().next;
        }

        let mut start = head;
        let mut size = 1;

        while size < length {
            let mut linked = Box::new(ListNode::new(0));
            let mut last_tail: *mut ListNode = &mut *linked;

            while start.is_some() {
                if start.as_ref().unwrap().next.is_none() {
                    unsafe {
                        // link the rest part
                        (*last_tail).next = start;
                    }
                    break;
                }

                let (first, second, rest) = Self::split_two_at(start, size);
                let (merged_part, tail) = Self::merge(first, second);

                // link the merged_part list to the linked list
                unsafe { (*last_tail).next = merged_part; }
                // update the `last_tail` to the linked's tail
                last_tail = tail;

                start = rest;
            }
            start = linked.next;
            size = size * 2;
        }

        start
    }

    /// merge two list, returning the merged list and the tail raw pointer of it, to prevent seek again
    fn merge(mut list1: List, mut list2: List) -> (List, *mut ListNode) {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut new_tail = &mut dummy_head;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                new_tail.next = list1.take();
                list1 = new_tail.next.as_mut().unwrap().next.take();
            } else {
                new_tail.next = list2.take();
                list2 = new_tail.next.as_mut().unwrap().next.take();
            }
            new_tail = new_tail.next.as_mut().unwrap();
        }

        new_tail.next = if list1.is_some() {
            list1
        } else {
            list2
        };

        while new_tail.next.is_some() {
            new_tail = new_tail.next.as_mut().unwrap();
        }

        // FIXME CANNOT use a raw pointer after value moved
        // `new_tail: Box<ListNode>` so its location would not be changed
        let tail_ptr: *mut ListNode = &mut **new_tail;

        (dummy_head.next, tail_ptr)
    }

    /// split a list into two sub list at the `at` index of the rest list,
    /// returning the (firist, second, rest)
    fn split_two_at(head: List, at: usize) -> (List, List, List) {
        if head.is_none() {
            return (None, None, None);
        }

        let mut head = head.unwrap();
        // because we need two mutable pointers, so here we have to use unsafe block,
        // but they are guaranteed safe here.
        unsafe {
            let mut slow: *mut ListNode = &mut *head;
            let mut fast: *mut ListNode = &mut **(*slow).next.as_mut().unwrap();
            let mut i = 1;
            while i < at && ((*fast).next.is_some() || (*slow).next.is_some()) {
                if (*slow).next.is_some() {
                    slow = &mut **(*slow).next.as_mut().unwrap();
                }
                if (*fast).next.is_some() {
                    if (*fast).next.as_ref().unwrap().next.is_some() {
                        fast = &mut **(*fast).next.as_mut().unwrap().next.as_mut().unwrap();
                    } else {
                        fast = &mut **(*fast).next.as_mut().unwrap();
                    }
                }
                i += 1;
            }
            (Some(head), (*slow).next.take(), (*fast).next.take())
        }
    }
}
