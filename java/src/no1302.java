import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class no1302 {
    int maxLvl, sum;

    @Test
    public void testDeepestLeavesSum_1() {
        TreeNode root = new TreeNode(1, null, null);
        assertEquals(1, deepestLeavesSum(root));
        assertEquals(0, deepestLeavesSum(null));
    }

    @Test
    public void testDeepestLeavesSum_2() {
        TreeNode children1 = new TreeNode(2, null, null);
        TreeNode children2 = new TreeNode(3, null, null);
        TreeNode root = new TreeNode(1, children1, children2);
        assertEquals(5, deepestLeavesSum(root));
    }

    @Test
    public void testDeepestLeavesSum_3() {
        TreeNode children2_1 = new TreeNode(4, null, null);
        TreeNode children2_2 = new TreeNode(5, null, null);
        TreeNode children1_1 = new TreeNode(2, children2_1, null);
        TreeNode children1_2 = new TreeNode(3, null, children2_2);
        TreeNode root = new TreeNode(1, children1_1, children1_2);
        assertEquals(9, deepestLeavesSum(root));
    }


    public int deepestLeavesSum(TreeNode root) {
        maxLvl = 0;
        sum = 0;
        dfs(root, 0);
        return sum;
    }

    void dfs(TreeNode r, int lvl) {
        if (r == null) {
            return;
        }
        if (r.left == null && r.right == null) {
            if (lvl > maxLvl) {
                sum = r.val;
                maxLvl = lvl;
            } else if (lvl == maxLvl) {
                sum += r.val;
            }
            return;
        }
        dfs(r.left, lvl + 1);
        dfs(r.right, lvl + 1);
    }
}

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {
    }

    TreeNode(int val) {
        this.val = val;
    }

    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}
