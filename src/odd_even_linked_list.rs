//! # 328. 奇偶链表
//!
//! 难度 中等
//!
//! 给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，而不是节点的值的奇偶性。
//!
//! 请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。
//!
//! ## 示例 1:
//!
//! ```text
//! 输入: 1->2->3->4->5->NULL
//! 输出: 1->3->5->2->4->NULL
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! 输入: 2->1->3->5->6->4->7->NULL
//! 输出: 2->3->6->7->1->5->4->NULL
//! ```
//!
//! ## 说明:
//!
//! - 应当保持奇数节点和偶数节点的相对顺序。
//! - 链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。
//!
//! See [leetcode](https://leetcode-cn.com/problems/odd-even-linked-list/)

use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut even_head = None;
        let mut even_tail = &mut even_head;

        let mut p = &mut head;
        while let Some(node) = p {
            if let Some(mut even) = node.next.take() {
                node.next = even.next.take();
                *even_tail = Some(even);
                even_tail = &mut even_tail.as_mut().unwrap().next;
            }

            if node.next.is_some() {
                p = &mut node.next;
            } else {
                // concat odd tail to even head
                node.next = even_head;
                break;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = |v| ListNode::into_vec(Solution::odd_even_list(ListNode::from_vec(v)));
        assert_eq!(vec![1,3,5,2,4], t(vec![1,2,3,4,5]));
        assert_eq!(vec![2,3,6,7,1,5,4], t(vec![2,1,3,5,6,4,7]));
        assert_eq!(vec![0i32;0], t(vec![]));
    }
}
