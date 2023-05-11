# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if root == None:
            return True
        leftheight = self.height(root.left)
        rightheight = self.height(root.right)
        if leftheight == -1 or rightheight == -1 or abs(leftheight - rightheight) > 1:
            return False
        return True

    def height(self, node) -> int:
        if node == None:
            return 0
        leftheight = self.height(node.left)
        rightheight = self.height(node.right)
        if leftheight == -1 or rightheight == -1 or abs(leftheight - rightheight) > 1:
            return -1
        return max(self.height(node.left), self.height(node.right)) + 1
