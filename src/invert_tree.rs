//! # 226. 翻转二叉树
//!
//! 难度 简单
//!
//! 翻转一棵二叉树。
//!
//! ## 示例：
//!
//! 输入：
//! ```plain
//!      4
//!    /   \
//!   2     7
//!  / \   / \
//! 1   3 6   9
//! ```
//!
//! 输出：
//! ```plain
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

impl TreeNode {
    fn swap(&mut self) {
        match (self.left.as_mut(), self.right.as_mut()) {
            (None, None) => (),
            _ => std::mem::swap(&mut self.left, &mut self.right),
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().map(|r| Solution::invert_node(&mut r.borrow_mut()));
        root
    }

    fn invert_node(n: &mut TreeNode) {
        match (n.left.as_ref(), n.right.as_ref()) {
            (None, None) => (),
            (Some(left), Some(right)) => {
                Solution::invert_node(&mut left.borrow_mut());
                Solution::invert_node(&mut right.borrow_mut());
            }
            (Some(v), _) | (_, Some(v)) => {
                Solution::invert_node(&mut v.borrow_mut());
            }
        };
        n.swap();
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
