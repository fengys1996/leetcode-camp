use std::{cell::RefCell, rc::Rc};

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
                    std::cmp::Ordering::Less => return,
                    std::cmp::Ordering::Equal => SUM += r.val,
                    std::cmp::Ordering::Greater => {
                        SUM = r.val;
                        MAX_LVL = lvl;
                    }
                }
            }
            Self::dfs(r.left.take(), lvl + 1);
            Self::dfs(r.right.take(), lvl + 1);
        }
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
}
