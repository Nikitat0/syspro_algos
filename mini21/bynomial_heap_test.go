package bynomial_heap

import (
	"testing"
)

func TestSimple(t *testing.T) {
	pq := NewPriorityQueue(1)
	if pq.PeekMin() != nil {
		t.Fatal()
	}
	pq.Insert(0, 0)
	if *pq.PeekMin() != 0 {
		t.Fatal()
	}
	if *pq.ExtractMin() != 0 {
		t.Fatal()
	}
	if pq.PeekMin() != nil {
		t.Fatal()
	}
}

func TestUpdatePriority(t *testing.T) {
	pq := NewPriorityQueue(3)
	pq.Insert(0, 30)
	if *pq.PeekMin() != 0 {
		t.Fatal()
	}
	pq.Insert(1, 20)
	if *pq.PeekMin() != 1 {
		t.Fatal()
	}
	pq.Insert(2, 10)
	if *pq.PeekMin() != 2 {
		t.Fatal()
	}
	pq.Insert(0, 5)
	if *pq.ExtractMin() != 0 {
		t.Fatal()
	}
	pq.Insert(0, 5)
	if *pq.PeekMin() != 0 {
		t.Fatal()
	}
	pq.Insert(2, 0)
	if *pq.ExtractMin() != 2 {
		t.Fatal()
	}
	if *pq.ExtractMin() != 0 {
		t.Fatal()
	}
	if *pq.ExtractMin() != 1 {
		t.Fatal()
	}
	if pq.ExtractMin() != nil {
		t.Fatal()
	}
}
