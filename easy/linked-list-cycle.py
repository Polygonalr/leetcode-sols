# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if head == None:
            return False
        curr = head.next
        visited = set()
        visited.add(head)
        while curr != None:
            if curr in visited:
                return True
            visited.add(curr)
            curr = curr.next
        return False
