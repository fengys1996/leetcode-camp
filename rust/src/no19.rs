#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut head_mut = head.as_ref();
        while let Some(node) = head_mut {
            length += 1;
            head_mut = node.next.as_ref();
        }
        if length < n {
            panic!("n is too long!");
        }
        let loop_num = length - n;
        let mut pre_head = Box::new(ListNode { val: 0, next: head });
        let mut curr = pre_head.as_mut();
        for _ in 0..loop_num {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = curr.next.take().unwrap().next;
        pre_head.next
    }

    #[allow(dead_code)]
    pub fn remove_nth_from_end_unsafe(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut pre_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast: *const _ = &pre_head;
        let mut slow = pre_head.as_mut().unwrap();
        let mut i = 0;
        while let Some(node) = unsafe { &*fast } {
            fast = &node.next;
            if i > n {
                slow = slow.next.as_mut().unwrap();
            }
            i += 1;
        }
        slow.next = slow.next.take().unwrap().next;
        pre_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::no19::{ListNode, Solution};

    #[test]
    fn test_no_19_solution() {
        let mut node1_in_first = Box::new(ListNode::new(1));
        let mut node2_in_first = Box::new(ListNode::new(3));
        let node3_in_first = Box::new(ListNode::new(5));
        node2_in_first.next = Some(node3_in_first);
        node1_in_first.next = Some(node2_in_first);

        let head = Solution::remove_nth_from_end(Some(node1_in_first), 3);
        let first_node = head.as_ref().unwrap();
        assert_eq!(3, first_node.val);
        let second_node = first_node.next.as_ref().unwrap();
        assert_eq!(5, second_node.val);
        assert_eq!(None, second_node.next);
    }

    #[test]
    fn test_no_19_solution_unsafe_impl() {
        let mut node1_in_first = Box::new(ListNode::new(1));
        let mut node2_in_first = Box::new(ListNode::new(3));
        let node3_in_first = Box::new(ListNode::new(5));
        node2_in_first.next = Some(node3_in_first);
        node1_in_first.next = Some(node2_in_first);

        let head = Solution::remove_nth_from_end_unsafe(Some(node1_in_first), 3);
        let first_node = head.as_ref().unwrap();
        assert_eq!(3, first_node.val);
        let second_node = first_node.next.as_ref().unwrap();
        assert_eq!(5, second_node.val);
        assert_eq!(None, second_node.next);
    }
}
