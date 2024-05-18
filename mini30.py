# https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/submissions/1261026820

class Solution:
    def sortItems(self, n, m, group, beforeItems):
        self.groups = [[] for _ in range(m)]
        self.groupOfTask = group
        for task in range(n):
            group = self.groupOfTask[task]
            if group != -1:
                self.groups[group].append(task)
                continue
            self.groupOfTask[task] = m
            self.groups.append([task])
            m += 1 

        self.groupsColor = [0] * m
        self.groupGraph = [set() for _ in range(m)]
        for i in range(n):
            if self.groupOfTask[i] == -1:
                continue
            for j in beforeItems[i]:
                v = self.groupOfTask[j]
                if v == -1:
                    continue
                u = self.groupOfTask[i]
                if u == v:
                    continue
                self.groupGraph[u].add(v)
        for i in range(m):
            if self.thereIsGroupCycle(i):
                return []

        self.taskColor = [0] * n
        self.taskGraph = beforeItems
        for i in range(n):
            if self.thereIsInGroupCycle(i, self.groupOfTask[i]):
                return []

        self.solution = [None] * n
        self.taskVisited = [False] * n
        self.groupVisited = [False] * m
        c = n
        for g in range(m):
            c = self.topsort(g, c)
        assert c == 0
        self.solution.reverse()
        return self.solution
    
    
    def thereIsGroupCycle(self, u):
        if self.groupsColor[u] == 2:
            return False
        if self.groupsColor[u] == 1:
            return True
        self.groupsColor[u] = 1
        for v in self.groupGraph[u]:
            if self.thereIsGroupCycle(v):
                return True
        self.groupsColor[u] = 2
        return False


    def thereIsInGroupCycle(self, u, g):
        if self.groupOfTask[u] != g or self.taskColor[u] == 2:
            return False
        if self.taskColor[u] == 1:
            return True
        self.taskColor[u] = 1
        for v in self.taskGraph[u]:
            if self.thereIsInGroupCycle(v, g):
                return True
        self.taskColor[u] = 2
        return False

    
    def topsort(self, g, c):
        if self.groupVisited[g]:
            return c
        self.groupVisited[g] = True

        for h in self.groupGraph[g]:
            c = self.topsort(h, c)

        for i in self.groups[g]:
            c = self.topsortGroup(i, g, c)
        return c
    
    def topsortGroup(self, u, g, c):
        if self.taskVisited[u] or self.groupOfTask[u] != g:
            return c
        self.taskVisited[u] = True
        for v in self.taskGraph[u]:
            c = self.topsortGroup(v, g, c)
        c -= 1
        self.solution[c] = u
        return c


s = Solution()
print(s.sortItems(8, 2, [-1, -1, 1, 0, 0, 1, 0, -1],
      [[], [6], [5], [6], [3, 6], [], [], []]))
print(s.sortItems(8, 2, [-1, -1, 1, 0, 0, 1, 0, -1],
      [[], [6], [5], [6], [3], [], [4], []]))
print(s.sortItems(8, 2, [-1, -1, 1, 0, 0, 1, 0, -1],
      [[], [6], [5], [6], [3, 6], [], [1], []]))
