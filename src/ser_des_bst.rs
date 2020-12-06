//! # 449. 序列化和反序列化二叉搜索树
//!
//! 难度 中等
//!
//! 序列化是将数据结构或对象转换为一系列位的过程，以便它可以存储在文件或内存缓冲区中，或通过网络连接链路传输，以便稍后在同一个或另一个计算机环境中重建。
//!
//! 设计一个算法来序列化和反序列化 二叉搜索树 。 对序列化/反序列化算法的工作方式没有限制。 您只需确保二叉搜索树可以序列化为字符串，并且可以将该字符串反序列化为最初的二叉搜索树。
//!
//! 编码的字符串应尽可能紧凑。
//!
//!  
//!
//! ## 示例 1：
//!
//! > 输入：root = [2,1,3]
//! > 输出：[2,1,3]
//!
//! ## 示例 2：
//!
//! > 输入：root = []
//! > 输出：[]
//!  
//! ## 提示：
//!
//! - 树中节点数范围是 [0, 104]
//! - 0 <= Node.val <= 104
//! - 题目数据 保证 输入的树是一棵二叉搜索树。
//!  
//!
//! 注意：不要使用类成员/全局/静态变量来存储状态。 你的序列化和反序列化算法应该是无状态的。
//!
//! See [leetcode](https://leetcode-cn.com/problems/serialize-and-deserialize-bst/)


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
}

use std::rc::Rc;
use std::cell::RefCell;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn ser(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        root.map_or(vec![None], |node| {
            let mut node = node.as_ref().borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let mut l = self.ser(left);
            let mut r = self.ser(right);
            let mut res = vec![];
            res.append(&mut l);
            res.append(&mut r);
            res.append(&mut vec![Some(node.val)]);
            res
        })
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.ser(root).iter().map(|v| {
            v.map_or("".to_string(), |v| format!("{}", v)) }).collect::<Vec<_>>().as_slice().join(" ")
    }

    fn des(&self, data: &mut Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        data.pop().and_then(|val| val).map(|val| {
            let right = self.des(data);
            let left = self.des(data);
            let node = TreeNode {
                val,
                left: left,
                right: right,
            };
            Rc::new(RefCell::new(node))
        })
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut d: Vec<Option<i32>> = data.split(' ').map(|s| {
            s.parse().ok()
        }).collect();
        self.des(&mut d)
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::{Codec, TreeNode};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test() {
        // let codec = Codec::new();
    }

    /// [2, 1, 3]
    /// to
    ///     2
    ///   /   \
    ///  1     3
    fn data_to_node_preorder(_data: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}
