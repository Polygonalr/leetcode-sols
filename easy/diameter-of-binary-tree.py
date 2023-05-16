# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        (_, ans) = self.diam(root)
        return ans
    def diam(self, root: Optional[TreeNode]) -> (int, int): # my height, best height
        if root == None:
            return (0, 0)
        (l, bl) = self.diam(root.left)
        (r, br) = self.diam(root.right)
        return (max(l, r) + 1, max(bl, br, l+r))
