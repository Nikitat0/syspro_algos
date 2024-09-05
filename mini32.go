// https://leetcode.com/problems/jump-game/submissions/1379967895

package mini32

func canJump(nums []int) bool {
	n := len(nums)
	i := 0
	for {
		nextIndex := i
		bestJumpLength := 0
		for j, jumpLength := range nums[i+1 : min(i+nums[i]+1, n)] {
			jumpLength += j
			j += i + 1
			if jumpLength >= bestJumpLength {
				bestJumpLength = jumpLength
				nextIndex = j
			}
		}
		if nextIndex == i {
			break
		}
		i = nextIndex
	}
	return i == n-1
}
