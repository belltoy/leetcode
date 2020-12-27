use std::rc::Rc;
use std::cell::RefCell;

/// LeetCode 题目中用到的树的节点结构
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    /// 从 `Vec<Option<i32>>` 生成树结构。一般建议使用宏 [`tree!`](tree)
    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        let root = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());

        for children in vec[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let Some(v) = children[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if children.len() > 1 {
                if let Some(v) = children[1] {
                    parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                    queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        root
    }
}

/// Generate a tree structure from a vec-like syntax.
///
/// For example:
///
/// ```no_run
/// #[macro_use] extern crate leetcode;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use leetcode::{tree, TreeNode};
///
/// let tree_node: Option<Rc<RefCell<TreeNode>>> = tree![4,2,7,1,3,6,9];
/// ```
/// will be expanded to an optional value: `Option<Rc<RefCell<TreeNode>>>` which with the following
/// tree structure:
///
/// ```text
///      4
///    /   \
///   2     7
///  / \   / \
/// 1   3 6   9
/// ```
///
/// And:
///
/// ```no_run
/// #[macro_use] extern crate leetcode;
/// use std::rc::Rc;
/// use std::cell::RefCell;
/// use leetcode::{tree, TreeNode};
///
/// let tree_node: Option<Rc<RefCell<TreeNode>>> = tree![4,2,7,null,null,6,9];
/// ```
///
/// will be expanded to an optional value: `Option<Rc<RefCell<TreeNode>>>` which with the following
/// tree structure:
///
/// ```text
///        4
///      /   \
///     2     7
///    / \   / \
///         6   9
/// ```
#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            TreeNode::from_vec(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let tree = tree![4,2,7,1,3,6,9];
        println!("{:?}", tree);
        assert_eq!(None::<Option<Rc<RefCell<TreeNode>>>>, tree![]);
    }
}
