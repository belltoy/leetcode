
/// LeetCode 里常用到的链表节点结构
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    /// 从 `Vec<i32>` 构建链表结构。建议使用宏 [`list!`](list)
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        vec_to_list(vec)
    }

    pub fn into_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut rest = list;
        std::iter::from_fn(move || {
            if let Some(node) = rest.as_mut() {
                let val = Some(node.val);
                rest = node.next.take();
                val
            } else {
                None
            }
        }).collect()
    }
}

pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.into_iter().rev().fold(None, |next, val| {
        Box::new(ListNode {
            val,
            next,
        }).into()
    })
}

/// Generate a linked list from a vec-like syntax.
///
/// For example:
///
/// ```no_run
/// #[macro_use] extern crate leetcode;
/// use leetcode::ListNode;
/// let head: Option<Box<ListNode>> = list![1,2,3,4];
/// // head:
/// // 1 -> 2 -> 3 -> 4 -> None
///
/// let head = list![1i32; 5];
/// // head:
/// // 1 -> 1 -> 1 -> 1 -> 1 -> None
/// ```
#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($elem:expr; $n:expr) => {
        $crate::ListNode::from_vec(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::ListNode::from_vec(vec![$($x),+])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let f = |v| ListNode::from_vec(v);
        let into = |v| ListNode::into_vec(v);
        assert_eq!(vec![0i32;0], into(f(vec![])));
        assert_eq!(vec![1,2,3], into(f(vec![1,2,3])));
    }

    #[test]
    fn test_list_macro() {
        let l1 = list![1,2,3,4];
        assert_eq!(vec![1,2,3,4], ListNode::into_vec(l1));
        assert_eq!(None::<Option<Box<ListNode>>>, list![]);
        assert_eq!(vec![1,1,1,1,1], ListNode::into_vec(list![1i32;5]));
        assert_eq!(vec![1], ListNode::into_vec(list![1]));
    }
}
