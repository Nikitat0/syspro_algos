package main

import (
	"bufio"
	. "mini31/graph"
	"strings"
	"unicode"
)

type Callgraph map[string][]string

func (g Callgraph) AddFunction(f string) {
	if _, ok := g[f]; !ok {
		g[f] = nil
	}
}

func (g Callgraph) AddDependency(caller, callee string) {
	g.AddFunction(callee)
	g[caller] = append(g[caller], callee)
}

func NewCallgraph() Callgraph {
	return Callgraph(make(map[string][]string))
}

func ReadCallgraph(scanner *bufio.Scanner) (Callgraph, error) {
	callgraph := NewCallgraph()
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		funcs := strings.FieldsFunc(line, func(r rune) bool {
			return unicode.IsSpace(r) || r == ':' || r == ','
		})
		caller := funcs[0]
		callgraph.AddFunction(caller)
		for _, callee := range funcs[1:] {
			callgraph.AddDependency(caller, callee)
		}
	}
	if scanner.Err() != nil {
		return nil, scanner.Err()
	}
	return callgraph, nil
}

func (g Callgraph) ToGraph() (Graph[string], map[string]int) {
	var graph Graph[string]
	mapping := make(map[string]int)
	for f := range g {
		mapping[f] = graph.AddVertex(f)
	}
	for caller, callees := range g {
		for _, callee := range callees {
			graph.AddEdge(mapping[caller], mapping[callee])
		}
	}
	return graph, mapping
}
