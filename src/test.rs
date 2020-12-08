use crate::util::ListNode;

type List = Option<Box<ListNode>>;

pub struct Solution;

impl Solution {
    /// Bottom Up Merge Sort
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
            println!("===== {} =====", size);
            let mut linked = Box::new(ListNode::new(0));
            let mut last_tail: *mut ListNode = &mut *linked;

            while start.is_some() {
                if start.as_ref().unwrap().next.is_none() {
                    print!("start: "); println_list(&start);
                    unsafe {
                        // link the rest part
                        (*last_tail).next = start;
                    }
                    break;
                }

                let (first, second, mut rest) = Self::split_two_at(start, size);

                print!("first: "); println_list(&first);
                print!("second: "); println_list(&second);
                print!("rest: "); println_list(&rest);
                let (merged_part, tail) = Self::merge(first, second);

                // link the merged_part list to the linked list
                unsafe { (*last_tail).next = merged_part; }
                // update the `last_tail` to the linked's tail
                last_tail = tail;

                start = rest;
            }
            start = linked.next;
            print!("merged: "); println_list(&start);
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

        let tail_ptr: *mut ListNode = &mut **new_tail;

        (dummy_head.next, tail_ptr)
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
                    println!("slow: {}, fast: {}", (*slow).val, (*fast).val);
                }
                (Some(head), (*slow).next.take(), (*fast).next.take())
            }
        }).unwrap()
    }
}

fn format_list(head: &List) -> String {
    use std::fmt::Write;
    let mut w = String::new();
    let mut head = head;
    while let Some(ref node) = head {
        write!(&mut w, "{}", node.val);
        head = &node.next;
        if head.is_some() {
            write!(&mut w, ", ");
        }
    }
    format!("[{}]", w)
}

fn println_list(l: &List) {
    println!("{}", format_list(l));
}
