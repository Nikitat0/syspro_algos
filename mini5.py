# https://leetcode.com/problems/h-index/submissions/1169605381

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
    def hIndex(self, citations):
        insertion_sort(citations)
        citations.reverse()
        for i, c in enumerate(citations):
            if c < i + 1:
                return i
        return len(citations)
