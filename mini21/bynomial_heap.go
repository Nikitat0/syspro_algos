package bynomial_heap

type PriorityQueue struct {
	trees   trees
	mapping []*tree
}

func NewPriorityQueue(size uint) PriorityQueue {
	return PriorityQueue{mapping: make([]*tree, size, size)}
}

func (q *PriorityQueue) Insert(element int, priority int) {
	if element >= len(q.mapping) || element < 0 {
		panic("No such element")
	}
	if q.mapping[element] != nil {
		q.mapping[element].updatePriority(priority, q.mapping)
		return
	}

	tree := newTree(element, priority)
	q.mapping[element] = tree

	var toInsert trees
	toInsert.appendTree(tree)
	q.trees.merge(toInsert)
}

func (q *PriorityQueue) PeekMin() *int {
	min := q.trees.peekMin()
	if min == nil {
		return nil
	}
	return &min.element
}

func (q *PriorityQueue) ExtractMin() *int {
	min := q.trees.extractMin()
	if min == nil {
		return nil
	}
	element, childs := min.decomposeTree()
	q.trees.merge(childs)
	q.mapping[element] = nil
	return &element
}

type tree struct {
	priority, element int
	order             uint
	parent            *tree
	childs            trees
	next              *tree
}

func newTree(element int, priority int) *tree {
	return &tree{element: element, priority: priority}
}

func (t tree) decomposeTree() (int, trees) {
	return t.element, t.childs
}

func (t *tree) updatePriority(priority int, mappings []*tree) {
	if priority > t.priority {
		panic("Reinserting with bigger priority is not allowed")
	}
	t.priority = priority
	for ; t.parent != nil && t.parent.priority > t.priority; t = t.parent {
		t.parent.priority, t.priority = t.priority, t.parent.priority
		t.parent.element, t.element = t.element, t.parent.element
		mappings[t.element] = t
		mappings[t.parent.element] = t
	}
}

func sumTrees(lhs, rhs *tree) *tree {
	if lhs.element > rhs.element {
		lhs, rhs = rhs, lhs
	}
	rhs.parent = lhs
	lhs.childs.appendTree(rhs)
	lhs.order++
	return lhs
}

type trees struct {
	head *tree
	tail *tree
}

func (t *trees) empty() bool {
	return t.head == nil
}

func (t *trees) appendTree(tree *tree) {
	if t.empty() {
		t.head = tree
		t.tail = tree
	} else {
		t.tail.next = tree
		t.tail = tree
	}
}

func (t *trees) appendTrees(other trees) {
	if other.empty() {
		return
	}
	if t.empty() {
		*t = other
		return
	}
	t.tail.next = other.head
	t.tail = other.tail
}

func (t *trees) popTree() *tree {
	defer func() {
		if t.head != nil {
			tmp := t.head.next
			t.head.next = nil
			t.head = tmp
		}
		if t.head == nil {
			t.tail = nil
		}
	}()
	return t.head
}

func (t *trees) peekMin() *tree {
	min := t.head
	for i := t.head; i != nil; i = i.next {
		if min.priority > i.priority {
			min = i
		}
	}
	return min
}

func (t *trees) extractMin() *tree {
	min := t.peekMin()
	if min == nil {
		return nil
	}
	var left trees
	for t.head != min {
		left.appendTree(t.popTree())
	}
	defer func() {
		left.appendTrees(*t)
		*t = left
	}()
	return t.popTree()
}

func (t *trees) merge(other trees) {
	var left trees
	var carry *tree
	ch := make(chan *tree)

	go func() {
		for !(t.empty() || other.empty()) {
			if t.head.order < other.head.order {
				ch <- t.popTree()
			} else {
				ch <- other.popTree()
			}
		}
		for !(t.empty()) && carry != nil {
			ch <- t.popTree()
		}
		for !(other.empty()) && carry != nil {
			ch <- other.popTree()
		}
		close(ch)
	}()

	for tree := range ch {
		tree.parent = nil
		if carry == nil {
			carry = tree
		} else if carry.order < tree.order {
			left.appendTree(carry)
			carry = tree
		} else if carry.order > tree.order {
			left.appendTree(tree)
		} else {
			carry = sumTrees(tree, carry)
		}
	}
	if carry != nil {
		left.appendTree(carry)
	}

	left.appendTrees(*t)
	left.appendTrees(other)
	*t = left
}
