// https://leetcode.com/problems/validate-binary-search-tree/submissions/1213537167

class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

fun isValidBST(root: TreeNode?): Boolean {
    return isValidBSTImpl(root, Long.MIN_VALUE, Long.MAX_VALUE)
}

fun isValidBSTImpl(
    node: TreeNode?,
    l: Long,
    r: Long,
): Boolean {
    return node?.run {
        node.`val` in l..r &&
            isValidBSTImpl(node.left, l, node.`val`.toLong() - 1) &&
            isValidBSTImpl(node.right, node.`val`.toLong() + 1, r)
    } ?: true
}
