//! # 226. 翻转二叉树
//!
//! 难度 简单
//!
//! 翻转一棵二叉树。
//!
//! ## 示例：
//!
//! 输入：
//! ```text
//!      4
//!    /   \
//!   2     7
//!  / \   / \
//! 1   3 6   9
//! ```
//!
//! 输出：
//! ```text
//!      4
//!    /   \
//!   7     2
//!  / \   / \
//! 9   6 3   1
//! ```
//!
//! ## 备注:
//! 这个问题是受到 [Max Howell](https://twitter.com/mxcl) 的 [原问题](https://twitter.com/mxcl/status/608682016205344768) 启发的 ：
//!
//! > 谷歌：我们 90％ 的工程师使用您编写的软件(Homebrew)，但是您却无法在面试时在白板上写出翻转二叉树这道题，这太糟糕了。

use std::rc::Rc;
use std::cell::RefCell;

use crate::TreeNode;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().map(|r| Solution::invert_node(&mut r.borrow_mut()));
        root
    }

    fn invert_node(n: &mut TreeNode) {
        if let Some(node) = n.left.as_ref() {
            Solution::invert_node(&mut node.borrow_mut());
        }
        if let Some(node) = n.right.as_ref() {
            Solution::invert_node(&mut node.borrow_mut());
        }
        std::mem::swap(&mut n.left, &mut n.right)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test() {
        let t = |v| Solution::invert_tree(v);
        assert_eq!(tree![4,7,2,9,6], t(tree![4,2,7,null,null,6,9]));
        assert_eq!(tree![4,7,2,9,6,3,1], t(tree![4,2,7,1,3,6,9]));
        assert_eq!(tree![4,7,2], t(tree![4,2,7]));
        assert_eq!(tree![4,7], t(tree![4,null,7]));
        assert_eq!(tree![4,null,2], t(tree![4,2]));
        assert_eq!(tree![], t(tree![]));
    }
}
