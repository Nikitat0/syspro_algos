package disjointsegments

import (
	"testing"
)

func TestLeftToRight(t *testing.T) {
	n := 5
	segs := NewDisjointSegments(n)
	for k := n - 1; k >= 0; k-- {
		segs.Union(k, n-1)
		for i := 0; i < k; i++ {
			if segs.Find(i) != i {
				t.Fatal()
			}
		}
		for i := k; i < n; i++ {
			if segs.Find(i) != n-1 {
				t.Fatal()
			}
		}
	}
}
