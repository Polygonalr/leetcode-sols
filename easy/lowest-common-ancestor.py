# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        if p.val > q.val:
            return self.lca(root, q, p)
        return self.lca(root, p, q)
    
    def lca(self, root, p, q) -> 'TreeNode':
        if p.val <= root.val and q.val >= root.val:
            return root
        if p.val >= root.val:
            return self.lca(root.right, p, q)
        return self.lca(root.left, p, q)

