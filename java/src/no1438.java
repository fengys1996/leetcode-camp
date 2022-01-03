import org.junit.jupiter.api.Test;

import java.util.TreeMap;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class no1438 {
    @Test
    void testLongestSubarray() {
        int[] nums = {8, 2, 4, 7};
        int res = longestSubarray(nums, 4);
        assertEquals(2, res);

        int[] nums1 = {10, 1, 2, 4, 7, 2};
        int res1 = longestSubarray(nums1, 5);
        assertEquals(4, res1);

        int[] nums2 = {2, 2, 2, 4, 4, 2, 2};
        int res2 = longestSubarray(nums2, 0);
        assertEquals(3, res2);
    }

    public int longestSubarray(int[] nums, int limit) {
        TreeMap<Integer, Integer> map = new TreeMap<>();
        int res = 0;
        int left = 0;
        int right = 0;
        int len = nums.length;
        while (right < len) {
            map.put(nums[right], map.getOrDefault(nums[right], 0) + 1);
            while (map.lastKey() - map.firstKey() > limit) {
                int updateNo = map.get(nums[left]) - 1;
                if (updateNo <= 0) {
                    map.remove(nums[left]);
                } else {
                    map.put(nums[left], updateNo);
                }
                left += 1;
            }
            res = Math.max(res, right - left + 1);
            right += 1;
        }
        return res;
    }
}
