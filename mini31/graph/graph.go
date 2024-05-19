package graph

import (
	. "mini31/stack"
)

type Graph[V any] struct {
	vertices []V
	adj      [][]int
}

func (g *Graph[V]) Size() int {
	return len(g.vertices)
}

func (g *Graph[V]) Vertices() []V {
	return g.vertices
}

func (g *Graph[V]) HasEdge(u, v int) bool {
	for _, w := range g.adj[v] {
		if v == w {
			return true
		}
	}
	return false
}

func (g *Graph[V]) AddVertex(vertex V) int {
	g.vertices = append(g.vertices, vertex)
	g.adj = append(g.adj, nil)
	return len(g.vertices) - 1
}

func (g *Graph[V]) AddEdge(u, v int) {
	g.adj[u] = append(g.adj[u], v)
}

func (g *Graph[V]) reverse() Graph[struct{}] {
	var rev Graph[struct{}]
	rev.vertices = make([]struct{}, g.Size())
	rev.adj = make([][]int, g.Size())
	for v, us := range g.adj {
		for _, u := range us {
			rev.AddEdge(u, v)
		}
	}
	return rev
}

func (g *Graph[V]) makeUsed() []bool {
	return make([]bool, g.Size())
}

func (g *Graph[V]) Sccs() ([][]int, []int) {
	rev := g.reverse()
	order := rev.Topsort()

	var sccs [][]int
	var stack Stack[int]
	vertexScc := make([]int, g.Size())
	used := g.makeUsed()
	for _, i := range order {
		if used[i] {
			continue
		}
		sccs = append(sccs, []int{})

		stack.Push(i)
		for len(stack) != 0 {
			u := stack.Pop()
			used[u] = true
			sccs[len(sccs)-1] = append(sccs[len(sccs)-1], u)
			vertexScc[u] = len(sccs) - 1
			for _, v := range g.adj[u] {
				if !used[v] {
					stack.Push(v)
				}
			}
		}
	}
	return sccs, vertexScc
}

func (g *Graph[V]) Topsort() []int {
	used := g.makeUsed()
	sorted := make([]int, g.Size())
	c := g.Size()
	for i := 0; i < g.Size(); i++ {
		c = g.topsort(i, used, sorted, c)
	}
	return sorted
}

func (g *Graph[V]) topsort(u int, used []bool, sorted []int, c int) int {
	if used[u] {
		return c
	}
	used[u] = true
	for _, u := range g.adj[u] {
		c = g.topsort(u, used, sorted, c)
	}
	c -= 1
	sorted[c] = u
	return c
}
