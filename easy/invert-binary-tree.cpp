/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    TreeNode* invertTree(TreeNode* root) {
        std::deque<TreeNode*> queue;
        if (root == nullptr) return nullptr;
        queue.push_back(root);
        while(queue.size() != 0) {
            TreeNode* front = queue.front();
            queue.pop_front();
            std::swap(front->left, front->right);
            if (front->left != nullptr) queue.push_back(front->left);
            if (front->right != nullptr) queue.push_back(front->right);
        }
        return root;
    }
};
