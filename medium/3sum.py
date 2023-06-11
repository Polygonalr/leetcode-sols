'''
Approach: Sort the input and reduce the problem to 2Sum

Algorithm applied here only works with a sorted array, and its runtime is > O(n log n). So, the array is sorted first.
The array is then iterated in order, with the current element being selected as the first number in the triplet to test
for. Two pointers from the start and end of the remaining subarray are initialized, and we will shift the left pointer
to the right and right pointer to the left if the current sum comprising of the current element, together with the two
elements pointed by the pointers is < 0 or > 0 respectively.

We then push all the valid triplets into the list and return it. Ideally a set should be returned, but the template code
is what it is...

Runtime: O(n^2) - Iterate the array once, and for every element visited there will be a single pass for the remaining
subarray.
'''

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        triplets = []
        prev = nums[0] - 1
        for i in range(len(nums) - 2):
            if nums[i] == prev:
                continue
            left, right = i+1, len(nums)-1
            while left < right:
                curr = nums[i] + nums[left] + nums[right]
                if curr < 0:
                    curr_left = nums[left]
                    while curr_left == nums[left] and (left < len(nums) and left < right):
                        left += 1
                elif curr > 0:
                    curr_right = nums[right]
                    while curr_right == nums[right] and (right >= 0 and left < right):
                        right -= 1
                else:
                    triplets.append((nums[i], nums[left], nums[right]))
                    curr_left, curr_right = nums[left], nums[right]
                    while curr_left == nums[left] and (curr_left < len(nums) and left < right):
                        left += 1
                    while curr_right == nums[right] and (curr_right >= 0 and left < right):
                        right -= 1
            prev = nums[i]
        #return list(map(lambda x: list(x), list(triplets)))
        return triplets