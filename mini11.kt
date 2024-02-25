// https://leetcode.com/problems/sort-colors/submissions/1184391905

class Solution {
    fun sortColors(nums: IntArray) {
        val n = nums.size

        var l = 0
        var r = n - 1
        var i = 0
        while (i <= r) {
            when (nums[i]) {
                0 -> {
                    nums.swap(l, i)
                    i++
                    l++
                }
                1 -> i++
                2 -> {
                    nums.swap(r, i)
                    r--
                }
            }
        }
    }
}

inline fun IntArray.swap(
    i: Int,
    j: Int,
) {
    val tmp = this[i]
    this[i] = this[j]
    this[j] = tmp
}
