
/// LeetCode 里常用到的链表节点结构
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
}
