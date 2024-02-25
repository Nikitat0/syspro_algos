# https://leetcode.com/problems/binary-search/submissions/1166530494

class Solution(object):
    def search(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: int
        """
        l, r = -1, len(nums)
        nums.append(None)
        while l + 1 != r:
            pivot = (l + r) // 2
            if nums[pivot] < target:
                l = pivot
            else:
                r = pivot
        return r if nums[r] == target else -1
