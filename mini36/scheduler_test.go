package mini36

import "testing"

func TestExample(t *testing.T) {
	tasks := []Task{
		{3, 25},
		{4, 10},
		{1, 30},
		{3, 50},
		{3, 20},
	}
	actual := runSolution(tasks)
	if actual != 20 {
		t.Fatal()
	}
}

var MY_TESTS = []test{
	newTest([]Task{
		{5, 50},
		{4, 40},
		{3, 30},
		{2, 20},
		{1, 10},
	}, 0),
	newTest([]Task{
		{2, 10},
		{2, 50},
		{2, 10},
		{1, 40},
		{2, 10},
	}, 30),
	newTest([]Task{
		{5, 20},
		{5, 10},
		{2, 40},
		{5, 30},
		{1, 30},
	}, 0),
	newTest([]Task{
		{2, 80},
		{1, 70},
		{2, 100},
		{2, 10},
		{1, 40},
		{3, 10},
		{4, 20},
	}, 120),
	newTest([]Task{
		{2, 68},
		{1, 20},
		{5, 18},
		{3, 71},
		{4, 82},
		{2, 99},
		{5, 38},
	}, 38),
}

func TestNaiveIsBad(t *testing.T) {
	for _, test := range MY_TESTS {
		if runSolution(test.tasks) != test.expected {
			t.Fatal()
		}
		if runNaiveSolution(test.tasks) == test.expected {
			t.Fatal()
		}
	}
}

func runSolution(tasks []Task) int {
	return CalcPenalty(MakeSchedule(tasks))
}

func runNaiveSolution(tasks []Task) int {
	return CalcPenalty(MakeScheduleNaive(tasks))
}

type test struct {
	tasks    []Task
	expected int
}

func newTest(tasks []Task, expected int) test {
	return test{
		tasks,
		expected,
	}
}
