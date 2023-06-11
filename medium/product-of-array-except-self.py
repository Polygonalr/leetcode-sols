'''
Approach: Build arrays of prefix and postfix multiplication

sol[0] = in[1] * in[2] * ... * in[n-1]
sol[1] = in[0] * in[2] * in[3] * ... * in[n-1]
...
sol[n-1] = in[0] * in[1] * in[2] * ... * in[n-2]

We can start with storing in[0] into sol[1], in[0] * in[1] into sol[2], in[0] * in[1] * in[2] into sol[3] and so on.
This can be done with a temporary variable in a single pass. We can then repeat the same backwards, but multiply the
temporary variable with the current value stored in each of the element in the sol array.

Runtime: O(n) as required by the problem
'''

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        if len(nums) == 0:
            return []
        elif len(nums) == 1:
            return nums

        sol = [1] * len(nums)
        curr = 1
        for i in range(len(nums)-1):
            curr *= nums[i]
            sol[i+1] = curr
        curr = 1
        for i in range(len(nums)-1, 0, -1):
            curr *= nums[i]
            sol[i-1] *= curr
        return sol