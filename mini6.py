# https://leetcode.com/problems/wiggle-sort-ii/submissions/1175834267

def insertion_sort_k(array, k):
    for i in range(k, len(array)):
        j = i
        while j - k >= 0 and array[j - k] > array[j]:
            array[j], array[j - k] = array[j - k], array[j]
            j -= k

def insertion_sort(array):
    n = len(array)
    ki = [1]
    while ki[-1] < n:
        i = len(ki)
        ki.append((4 ** i) + 3 * (2 ** (i - 1)) + 1)
    ki.pop()
    ki.reverse()
    for k in ki:
        insertion_sort_k(array, k)

class Solution(object):
    
    def wiggleSort(self, nums):
        buf = nums[0:]
        insertion_sort(buf)
        n = len(nums)
        i = 0
        if n % 2:
            nums[n - 1] = buf.pop(0)
            n -= 1
        while i != n // 2:
            nums[2 * i] = buf[n // 2 - i - 1]
            nums[2 * i + 1] = buf[n - i - 1]
            i += 1
