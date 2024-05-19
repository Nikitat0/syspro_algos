package main

import (
	"fmt"
	"slices"
	"sort"
	"strings"
)

type Report struct {
	largestComponents []RecursiveComponent
	recursive         []string
}

func (r Report) String() string {
	var builder strings.Builder
	switch len(r.largestComponents) {
	case 0:
		fmt.Fprintln(&builder, "There is no recursive components")
	case 1:
		fmt.Fprint(&builder, "Largest recursive component is ")
	default:
		fmt.Fprintln(&builder, "Largest recursive component are:")
	}
	for _, rc := range r.largestComponents {
		fmt.Fprintln(&builder, rc.String())
	}
	for _, rf := range r.recursive {
		fmt.Fprintln(&builder, rf, "is recursive function")
	}
    return builder.String()[:builder.Len()-1]
}

func BuildReport(callgraph Callgraph) Report {
	graph, _ := callgraph.ToGraph()
	sccs, _ := graph.Sccs()
	var report Report
	var maxLen int
	for _, scc := range sccs {
        if !isRecursiveComponent(scc, graph.HasEdge) {
            continue
        }
        for _, fnId := range scc {
            report.addRecursiveFunction(graph.Vertices()[fnId])
        }
		l := len(scc)
		if l > maxLen {
			maxLen = l
            report.resetRecursiveComponents()
		}
        if l == maxLen {
            var rc RecursiveComponent
            for _, fnId := range scc {
                rc = append(rc, graph.Vertices()[fnId])
            }
            sort.Strings(rc)
            report.addRecursiveComponent(rc)
        }
	}
	slices.SortFunc(report.largestComponents, func(a, b RecursiveComponent) int {
		return slices.Compare(a, b)
	})
	sort.Strings(report.recursive)
	return report
}

func (r *Report) addRecursiveComponent(rc RecursiveComponent) {
	r.largestComponents = append(r.largestComponents, rc)
}

func (r *Report) addRecursiveFunction(fn string) {
	r.recursive = append(r.recursive, fn)
}

func (r *Report) resetRecursiveComponents() {
    r.largestComponents = nil
}

func isRecursiveComponent(scc []int, hasEdge func (u, v int) bool) bool {
    l := len(scc)
    if l > 1 {
        return true
    }
    return hasEdge(scc[0], scc[0])
}

type RecursiveComponent []string

func (c RecursiveComponent) String() string {
	var builder strings.Builder
	fmt.Fprint(&builder, "{")
	for i, fn := range c {
		if i != 0 {
			fmt.Fprint(&builder, ", ")
		}
		fmt.Fprint(&builder, fn)
	}
	fmt.Fprint(&builder, "}")
	return builder.String()
}
