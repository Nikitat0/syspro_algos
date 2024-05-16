# https://leetcode.com/problems/is-graph-bipartite/submissions/1259242557

class Solution:
    def isBipartite(self, graph) -> bool:
        graph = complement_graph(graph)
        n = len(graph)
        colors = [0] * n

        stack = []
        for i in range(n):
            if colors[i]:
                continue
            
            colors[i] = 1
            stack.append(i)
            while stack:
                u = stack.pop()
                c = next_color(colors[u])
                for v in graph[u]:
                    if colors[v] == c:
                        continue
                    if colors[v] != 0:
                        return False
                    stack.append(v)
                    colors[v] = c
        return True


def next_color(c):
    return c % 2 + 1


def complement_graph(graph):
    n = len(graph)
    matrix = [[False] * n for _ in range(n)]
    for i in range(n):
        for j in graph[i]:
            matrix[i][j] = True
            matrix[j][i] = True

    graph = []
    for i in range(n):
        adj = []
        for j in range(n):
            if matrix[i][j]:
                adj.append(j)
        graph.append(adj)
    return graph
