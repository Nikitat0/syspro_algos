package stack

type Stack[V any] []V

func (s *Stack[V]) Top() V {
    return (*s)[len(*s)-1]
}

func (s *Stack[V]) Empty() bool {
    return len(*s) == 0
}

func (s *Stack[V]) Push(value V) {
    *s = append(*s, value)
}

func (s *Stack[V]) Pop() V {
    value := s.Top()
    *s = (*s)[:len(*s)-1]
    return value
}
