// https://leetcode.com/problems/trim-a-binary-search-tree/submissions/1215941847

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
    if root.Val < low {
        return trimBST(root.Right, low, high)
    }
    if high < root.Val {
        return trimBST(root.Left, low, high)
    }
	root.Left = trimBST(root.Left, low, high)
	root.Right = trimBST(root.Right, low, high)
	return root
}
