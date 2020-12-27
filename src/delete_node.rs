//! # 剑指 Offer 18. 删除链表的节点
//!
//! 难度 简单
//!
//! 给定单向链表的头指针和一个要删除的节点的值，定义一个函数删除该节点。
//!
//! 返回删除后的链表的头节点。
//!
//! 注意：此题对比原题有改动
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: head = [4,5,1,9], val = 5
//! 输出: [4,1,9]
//! 解释: 给定你链表中值为 5 的第二个节点，那么在调用了你的函数之后，该链表应变为 4 -> 1 -> 9.
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: head = [4,5,1,9], val = 1
//! 输出: [4,5,9]
//! 解释: 给定你链表中值为 1 的第三个节点，那么在调用了你的函数之后，该链表应变为 4 -> 5 -> 9.
//! ```
//!
//! ## 说明：
//!
//! * 题目保证链表中节点的值互不相同
//! * 若使用 C 或 C++ 语言，你不需要 free 或 delete 被删除的节点
//!
//! See [leetcode](https://leetcode-cn.com/problems/shan-chu-lian-biao-de-jie-dian-lcof/)

use crate::ListNode;

pub struct Solution;

impl Solution {

    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{ next: head, val: 0 }));
        let mut node = &mut dummy;
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            if node.as_ref().unwrap().next.as_ref().unwrap().val == val {
                let n = node.as_mut().unwrap().next.take();
                node.as_mut().unwrap().next = n.unwrap().next;
                break;
            }
            node = &mut node.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        let cases = vec![
            (vec![4,1,9], (list![4,5,1,9], 5)),
            (vec![4,5,9], (list![4,5,1,9], 1)),
            (vec![], (list![], 1)),
            (vec![4,1,9], (list![4,1,9], 3)),
        ];

        for (expect, (head, val)) in cases {
            assert_eq!(expect, ListNode::into_vec(Solution::delete_node(head, val)));
        }
    }
}
