# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def middleNode(self, head: Optional[ListNode]) -> Optional[ListNode]:
        count = 0
        curr = head
        while curr != None:
            curr = curr.next
            count += 1
        count -= 1
        curr = head
        for _ in range(ceil(count/2)):
            curr = curr.next
        return curr

