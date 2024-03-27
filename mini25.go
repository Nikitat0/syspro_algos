// https://leetcode.com/problems/trim-a-binary-search-tree/submissions/1215393288

package mini25

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func trimBST(root *TreeNode, low int, high int) *TreeNode {
	if root == nil {
		return nil
	}
	if low <= root.Val && root.Val <= high {
		root.Left = trimBST(root.Left, low, high)
		root.Right = trimBST(root.Right, low, high)
		return root
	}
	for _, repl := range []*TreeNode{root.Left, root.Right} {
		repl = trimBST(repl, low, high)
		if repl == nil {
			continue
		}
		return repl
	}
	return nil
}
