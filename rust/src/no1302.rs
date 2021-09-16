use std::{cell::RefCell, cmp::Ordering, collections::VecDeque, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

static mut MAX_LVL: i32 = 0;
static mut SUM: i32 = 0;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub unsafe fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        MAX_LVL = 0;
        SUM = 0;
        Self::dfs(root, 0);
        SUM
    }

    unsafe fn dfs(tree_node: Option<Rc<RefCell<TreeNode>>>, lvl: i32) {
        if let Some(node) = tree_node {
            let mut r = node.borrow_mut();
            if r.left.is_none() && r.right.is_none() {
                match lvl.cmp(&MAX_LVL) {
                    Ordering::Less => return,
                    Ordering::Equal => SUM += r.val,
                    Ordering::Greater => {
                        SUM = r.val;
                        MAX_LVL = lvl;
                    }
                }
            }
            Self::dfs(r.left.take(), lvl + 1);
            Self::dfs(r.right.take(), lvl + 1);
        }
    }

    #[allow(dead_code)]
    pub fn deepest_leaves_sum_greater(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_lvl = 0;
        let mut sum = 0;
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap().clone();
            let depth = node.1;
            let node = node.0.borrow_mut();
            if let Some(new_node) = node.left.clone() {
                queue.push_back((new_node, depth + 1));
            }
            if let Some(new_node) = node.right.clone() {
                queue.push_back((new_node, depth + 1));
            }
            match depth.cmp(&max_lvl) {
                Ordering::Less => {}
                Ordering::Equal => sum += node.val,
                Ordering::Greater => {
                    max_lvl = depth;
                    sum = node.val;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use crate::no1302::Solution;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_deepest_leaves_sum_1() {
        unsafe {
            assert_eq!(0, Solution::deepest_leaves_sum(None));
            let root = TreeNode::new(1);
            assert_eq!(
                1,
                Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(root))))
            );
        }
    }

    #[test]
    fn test_deepest_leaves_sum_2() {
        unsafe {
            let mut root = TreeNode::new(1);
            let mut children1_1 = TreeNode::new(2);
            let mut children1_2 = TreeNode::new(3);
            let children2_1 = TreeNode::new(4);
            let children2_2 = TreeNode::new(5);
            children1_1.left = Some(Rc::new(RefCell::new(children2_1)));
            children1_2.right = Some(Rc::new(RefCell::new(children2_2)));
            root.left = Some(Rc::new(RefCell::new(children1_1)));
            root.right = Some(Rc::new(RefCell::new(children1_2)));
            assert_eq!(
                9,
                Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(root))))
            );
        }
    }

    #[test]
    fn test_deepest_leaves_sum_grater_1() {
        assert_eq!(0, Solution::deepest_leaves_sum_greater(None));
        let root = TreeNode::new(1);
        assert_eq!(
            1,
            Solution::deepest_leaves_sum_greater(Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn test_deepest_leaves_sum_greater_2() {
        let mut root = TreeNode::new(1);
        let mut children1_1 = TreeNode::new(2);
        let mut children1_2 = TreeNode::new(3);
        let children2_1 = TreeNode::new(4);
        let children2_2 = TreeNode::new(5);
        children1_1.left = Some(Rc::new(RefCell::new(children2_1)));
        children1_2.right = Some(Rc::new(RefCell::new(children2_2)));
        root.left = Some(Rc::new(RefCell::new(children1_1)));
        root.right = Some(Rc::new(RefCell::new(children1_2)));
        assert_eq!(
            9,
            Solution::deepest_leaves_sum_greater(Some(Rc::new(RefCell::new(root))))
        );
    }
}
