#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let node1_next = node1.next.take();
                    node1.next = Solution::merge_two_lists(node1_next, Some(node2));
                    Some(node1)
                } else {
                    let node2_next = node2.next.take();
                    node2.next = Solution::merge_two_lists(Some(node1), node2_next);
                    Some(node2)
                }
            }
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn merge_two_lists1(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut pre_head = Box::new(ListNode::new(-1));
        let mut pre = &mut pre_head;

        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            if node1.val < node2.val {
                pre.next = l1;
                pre = pre.next.as_mut().unwrap();
                l1 = pre.next.take();
            } else {
                pre.next = l2;
                pre = pre.next.as_mut().unwrap();
                l2 = pre.next.take();
            }
        }
        pre.next = if l1.is_some() { l1 } else { l2 };
        pre_head.next
    }
}

type MergeTwoListFn = fn(Option<Box<ListNode>>, Option<Box<ListNode>>) -> Option<Box<ListNode>>;

#[allow(dead_code)]
fn do_test_no_21_solution(func: MergeTwoListFn) {
    // mock data
    let mut node1_in_first = Box::new(ListNode::new(1));
    let mut node2_in_first = Box::new(ListNode::new(3));
    let node3_in_first = Box::new(ListNode::new(5));
    node2_in_first.next = Some(node3_in_first);
    node1_in_first.next = Some(node2_in_first);

    let mut node1_in_second = Box::new(ListNode::new(2));
    let mut node2_in_second = Box::new(ListNode::new(4));
    let node3_in_second = Box::new(ListNode::new(6));
    node2_in_second.next = Some(node3_in_second);
    node1_in_second.next = Some(node2_in_second);

    // start test
    let mut result = func(Some(node1_in_first), Some(node1_in_second));
    let mut result_vec = Vec::new();
    while let Some(x) = result {
        result_vec.push(x.val);
        result = x.next;
    }
    assert_eq!(result_vec, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_no_21_solution() {
    do_test_no_21_solution(Solution::merge_two_lists);
    do_test_no_21_solution(Solution::merge_two_lists1);
}
