# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        ans = 0
        def dfs(node,cnt):
            if node is None:
                return

            cnt += 1
            nonlocal ans
            ans = max(cnt,ans)

            dfs(node.left,cnt)
            dfs(node.right,cnt)
        dfs(root,0)
        return ans