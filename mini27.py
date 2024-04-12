# https://leetcode.com/problems/maximum-frequency-stack/submissions/1230423337

class FreqStack():

    def __init__(self):
        self.__freq = dict()
        self.__buckets = list()

    def push(self, e):
        f = self.__freq.get(e, 0)
        self.__freq[e] = f + 1
        if len(self.__buckets) == f:
            self.__buckets.append([])
        self.__buckets[f].append(e)
        
    def pop(self):
        if not self.__buckets:
            return None
        e = self.__buckets[-1].pop()
        self.__freq[e] -= 1
        if not self.__buckets[-1]:
            self.__buckets.pop()
        return e
