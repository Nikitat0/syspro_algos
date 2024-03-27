// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/submissions/1215374494

package main

import (
	"fmt"
	"strings"
	uni "unicode"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct {
	strings.Builder
}

func Constructor() Codec {
	return Codec{}
}

func (c *Codec) serialize(root *TreeNode) string {
	defer c.Reset()
	c.serializeImpl(root)
	return c.String()
}

func (c *Codec) serializeImpl(node *TreeNode) {
	if node == nil {
		fmt.Fprint(c, "x")
		return
	}
	fmt.Fprintf(c, "(%d", node.Val)
	c.serializeImpl(node.Left)
	c.serializeImpl(node.Right)
	fmt.Fprintf(c, ")")
}

func (this *Codec) deserialize(str string) *TreeNode {
	return deserializeImpl(strings.NewReader(str))
}

func deserializeImpl(str *strings.Reader) *TreeNode {
	if r, _, _ := str.ReadRune(); r == 'x' {
		return nil
	}
	var node TreeNode
	k := -1
	if m, _, _ := str.ReadRune(); m != '-' {
		str.UnreadRune()
		k = 1
	}
	for d := '0'; uni.IsDigit(d); d, _, _ = str.ReadRune() {
		node.Val *= 10
		node.Val += int(d - '0')
	}
	node.Val *= k
	str.UnreadRune()
	node.Left = deserializeImpl(str)
	node.Right = deserializeImpl(str)
	str.ReadRune()
	return &node
}

func main() {
	var c Codec
	fmt.Println(c.serialize(c.deserialize("(5(-7x(5xx))x)")))
}
