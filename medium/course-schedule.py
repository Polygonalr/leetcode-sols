'''
Approach: Checking cycles in directed graph

Build a graph from the input, and run DFS to check for cycles. DFS works by marking nodes as visited along its path, and
if the DFS hit a node that has been visited before, it will return False.

For nodes with no children, they will be marked as global_visited[node] = True eventually, and all the nodes that was
traversed in the current DFS cycle will be marked the same as well - this helped with optimizing the problem as any node
that are marked as such after any DFS cycle are guaranteed to have no cycles. That is, if any traversal hits any of the
marked nodes, it is certain that the previous node is also not part of a cycle. This helped prune the DFS required to
solve the problem.

Runtime: O(n) where n is the number of nodes
'''

class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        self.graph = [[] for _ in range(numCourses)] # Mapping of {Course: List of courses it is a pre-req for}
        self.visited = [False for _ in range(numCourses)]
        self.global_visited = [False for _ in range(numCourses)]

        # Build graph
        for e in prerequisites:
            self.graph[e[0]].append(e[1])

        for i in range(numCourses):
            if not self.dfs(i):
                return False
        return True

    def dfs(self, curr):
        if self.visited[curr]:
            return False
        if self.global_visited[curr]:
            return True
        self.visited[curr] = True
        for e in self.graph[curr]:
            if not self.dfs(e):
                return False
        self.global_visited[curr] = True
        self.visited[curr] = False
        return True