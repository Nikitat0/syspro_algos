package mini36

import (
	"cmp"
	. "mini36/disjointsegments"
	"slices"
)

type Task struct {
	deadline, penalty int
}

func CalcPenalty(schedule []Task) int {
	c := 0
	for i, task := range schedule {
		if task.deadline < i+1 {
			c += task.penalty
		}
	}
	return c
}

func MakeSchedule(tasks []Task) []Task {
	n := len(tasks)
	tasks = slices.Clone(tasks)
	slices.SortFunc(tasks, func(a, b Task) int {
		return cmp.Compare(a.penalty, b.penalty) * -1
	})
	segs := NewDisjointSegments(n)
	slots := make([]int, n)
	for i := range slots {
		slots[i] = -1
	}
	for i, task := range tasks {
		slot := task.deadline - 1
		for slots[slot] != -1 {
			nextSlot := segs.Leftmost(slot) - 1
			if nextSlot == -1 {
				nextSlot += n
			}
			segs.Union(nextSlot, slot)
			slot = nextSlot
		}
		slots[slot] = i
	}
	schedule := make([]Task, n)
	for i, taskIdx := range slots {
		schedule[i] = tasks[taskIdx]
	}
	return schedule
}

func MakeScheduleNaive(tasks []Task) []Task {
	tasks = slices.Clone(tasks)
	slices.SortFunc(tasks, func(a, b Task) int {
		cmpRes := cmp.Compare(a.penalty, b.penalty) * -1
		if cmpRes == 0 {
			cmpRes = cmp.Compare(a.deadline, b.deadline)
		}
		return cmpRes
	})
	return tasks
}
