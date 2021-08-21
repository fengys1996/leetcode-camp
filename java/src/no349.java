import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

public class no349 {

    @Test
    public void test_intersect() {
        int[] nums1 = {1, 2, 2, 5};
        int[] nums2 = {2, 2};
        int[] expect = {2};
        int[] res = new no349().intersect(nums1, nums2);
        assertArrayEquals(res, expect);
    }

    public int[] intersect(int[] nums1, int[] nums2) {
        Set<Integer> set = new HashSet<>();
        for (int item : nums2) {
            set.add(item);
        }
        Set<Integer> res_set = new HashSet<>();
        for (int item : nums1) {
            if (set.contains(item)) {
                res_set.add(item);
            }
        }
        int[] res = new int[res_set.size()];
        int index = 0;
        for (int item : res_set) {
            res[index++] = item;
        }
        return res;
    }
}
