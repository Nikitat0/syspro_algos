package disjointsegments

type DisjointSegments struct {
	disjointSet []segmentDescriptor
}

type segmentDescriptor struct {
	parent, rank, leftmost int
}

func NewDisjointSegments(size int) DisjointSegments {
	segs := DisjointSegments{
		disjointSet: make([]segmentDescriptor, size),
	}
	for i := range segs.disjointSet {
		segs.disjointSet[i].leftmost = i
		segs.disjointSet[i].parent = -1
	}
	return segs
}

func (s *DisjointSegments) Find(seg int) int {
	parent := s.parent(seg)
	if parent == -1 {
		return seg
	}
	topmost := s.Find(parent)
	s.disjointSet[seg].parent = topmost
	return topmost
}

func (s *DisjointSegments) Leftmost(seg int) int {
	return s.leftmost(s.Find(seg))
}

func (s *DisjointSegments) Union(seg1, seg2 int) {
	seg1, seg2 = s.Find(seg1), s.Find(seg2)
	if seg1 == seg2 {
		return
	}
	leftmost := s.leftmost(seg1)
	if s.rank(seg1) > s.rank(seg2) {
		seg1, seg2 = seg2, seg1
	}
	s.disjointSet[seg1].parent = seg2
	s.disjointSet[seg2].leftmost = leftmost
	s.disjointSet[seg2].rank = max(s.rank(seg1)+1, s.rank(seg2))
}

func (s *DisjointSegments) leftmost(seg int) int {
	return s.disjointSet[seg].leftmost
}

func (s *DisjointSegments) parent(seg int) int {
	return s.disjointSet[seg].parent
}

func (s *DisjointSegments) rank(seg int) int {
	return s.disjointSet[seg].rank
}
