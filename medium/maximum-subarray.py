class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        n = len(nums)
        memo = [-1] * n
        memo[0] = nums[0]
        best = memo[0]
        for i in range(1, n):
            if memo[i-1] > 0:
                memo[i] = memo[i-1] + nums[i]
            else:
                memo[i] = nums[i]
            best = max(memo[i], best)
        return best
