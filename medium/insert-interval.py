'''
Approach: Narrow down the problem and solve it.

First, we split the intervals into three parts, two of which are intervals that are complete to the left and right of
the new interval. They are not affected by the new interval insertion and will be added back to the solution as it is.
The third part is the intervals that are affected by the new interval insertion. The new interval will be merged with
the first and last interval in this part.

We then merge all three parts after processing the third part and return the solution.

Runtime: O(n)
'''

class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        left_intervals = [x for x in intervals if x[1] < newInterval[0]]
        right_intervals = [x for x in intervals if x[0] > newInterval[1]]
        middle_intervals = [x for x in intervals if x[1] >= newInterval[0] and x[0] <= newInterval[1]]

        if len(middle_intervals) == 0:
            sol = [
                [newInterval[0], newInterval[1]]
            ]
        else:
            new_interval = [
                min(middle_intervals[0][0], newInterval[0]),
                max(middle_intervals[-1][1], newInterval[1])
            ]
            sol = [new_interval]

        return left_intervals + sol + right_intervals
