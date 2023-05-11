class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        memo = dict()
        for n in nums:
            if n not in memo:
                memo[n] = 1
            else:
                memo[n] += 1
            if memo[n] > len(nums)/2:
                return n
        return -1
