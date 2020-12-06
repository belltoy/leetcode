// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self { ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl ListNode {
    pub fn len(&self) -> usize {
        let mut count = 1;
        let mut node = Some(self);
        while node.is_some() {
            count += 1;
            node = node.as_ref().unwrap().next.as_deref();
        }
        count
    }
}

/// dummy head and the maintained raw pointer of its tail
struct Tailed {
    node: Box<ListNode>,
    tail: *mut ListNode,  // TODO use NonNull here
}

impl Tailed {
    pub fn new() -> Self {
        let mut tailed = Self {
            node: Box::new(ListNode::new(0)),
            tail: std::ptr::null_mut(),
        };
        tailed.tail = &mut *tailed.node;
        tailed
    }

    pub fn into_inner(self) -> Option<Box<ListNode>> {
        self.node.next
    }

    pub fn extend(&mut self, list: Option<Box<ListNode>>) {
        list.map(|node| unsafe {
            (*self.tail).next = Some(node);
            self.tail = &mut **(*self.tail).next.as_mut().unwrap();
            // move tail pointer to the new end
            while let Some(ref mut node) = (*self.tail).next.as_mut() {
                self.tail = &mut ***node;
            }
        });
    }

    // sorted merge the two given list to the inner, keep the tail to the last node
    pub fn merge(&mut self, mut list1: List, mut list2: List) {
        unsafe {
            while let (Some(ref node1), Some(ref node2)) = (&list1, &list2) {
                if node1.val < node2.val {
                    (*self.tail).next = list1.take();
                    self.tail = &mut **(*self.tail).next.as_mut().unwrap();
                    list1 = (*self.tail).next.take();
                } else {
                    (*self.tail).next = list2.take();
                    self.tail = &mut **(*self.tail).next.as_mut().unwrap();
                    list2 = (*self.tail).next.take();
                }
            }

            if list1.is_some() {
                self.extend(list1);
            } else {
                self.extend(list2);
            }
        }
    }
}

type List = Option<Box<ListNode>>;
impl Solution {

    /// Bottom Up Merge Sort
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let length = head.as_ref().map(|list| list.len())?;

        let mut start = head;
        let mut size = 1;

        while size < length {
            let mut tailed = Tailed::new();

            while start.is_some() {
                if start.as_ref().unwrap().next.is_none() {
                    // link the rest part
                    tailed.extend(start);
                    break;
                }

                let (first, second, mut rest) = Self::split_two_at(start, size);
                tailed.merge(first, second);

                start = rest;
            }
            start = tailed.into_inner();
            size = size * 2;
        }
        start
    }

    /// split a list into two sub list at the `at` index of the rest list,
    /// returning the (firist, second, rest)
    fn split_two_at(mut head: List, at: usize) -> (List, List, List) {
        if head.is_none() {
            return (None, None, None);
        }

        head.map(|mut head| {
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
        }).unwrap()
    }
}