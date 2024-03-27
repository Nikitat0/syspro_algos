// https://leetcode.com/problems/binary-tree-right-side-view/submissions/1215383333

package mini23

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
	var cur, next []*TreeNode
	var righties []int
    if root != nil {
        next = append(next, root)
    }
	for len(next) > 0 {
		cur, next = next, cur[:0]
		righties = append(righties, cur[len(cur)-1].Val)
		for _, node := range cur {
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
	}
	return righties
}
