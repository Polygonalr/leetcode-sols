# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root == None: return None
        queue = [root]
        while len(queue) != 0:
            curr = queue[0]
            queue = queue[1:]
            curr.left, curr.right = curr.right, curr.left
            if curr.left != None: queue.append(curr.left)
            if curr.right != None: queue.append(curr.right)
        return root
