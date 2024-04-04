//https://leetcode.com/problems/balance-a-binary-search-tree/submissions/1222676140

package main

import "fmt"

func main() {
	root := TreeNode{
		Val:   5,
		Left:  &TreeNode{
            Val: 3,
            Left: &TreeNode {
                Val: 2,
                Left: &TreeNode {
                    Val: 1,
                },
            },
            Right: &TreeNode {
                Val: 4,
            },
        },
		Right: nil,
	}
	fmt.Println(emit(&root))
	fmt.Println(balanceBST(&root))
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func emit(t *TreeNode) []int {
	var seq []int
	emitImpl(t, &seq)
	return seq
}

func emitImpl(t *TreeNode, seq *[]int) {
	if t == nil {
		return
	}
	emitImpl(t.Left, seq)
	*seq = append(*seq, t.Val)
	emitImpl(t.Right, seq)
}

func buildTree(seq []int) *TreeNode {
    return buildTreeImpl(seq, false)
}

func buildTreeImpl(seq []int, toRight bool) *TreeNode {
    n := len(seq)
    if n == 0 {
        return nil;
    }
    if n % 2 != 0 {
        return &TreeNode {
            Val: seq[n / 2],
            Left: buildTree(seq[:n / 2]),
            Right: buildTree(seq[n / 2 + 1:]),
        }
    }
    if toRight {
        return &TreeNode {
            Val: seq[n / 2],
            Left: buildTreeImpl(seq[:n / 2], false),
            Right: buildTreeImpl(seq[n / 2 + 1:], false),
        }
    } else {
        return &TreeNode {
            Val: seq[n / 2 - 1],
            Left: buildTreeImpl(seq[:n / 2 - 1], false),
            Right: buildTreeImpl(seq[n / 2:], false),
        }
    }
}

func balanceBST(root *TreeNode) *TreeNode {
    return buildTree(emit(root))
}
