# Definition for a pair.
# class Pair:
#     def __init__(self, key: int, value: str):
#         self.key = key
#         self.value = value
class Solution:
    def _merge(self, a: List[Pair], b: List[Pair]) -> List[Pair]:
        result : List[Pair] = []
        i, j = 0, 0
        while i < len(a) and j < len(b):
            if a[i].key <= b[j].key:
                result.append(a[i])
                i = i + 1
            else:
                result.append(b[j])
                j += 1
        result.extend(a[i:])
        result.extend(b[j:])
        return result

    def mergeSort(self, pairs: List[Pair]) -> List[Pair]:
        m = len(pairs) // 2
        if m == 0:
            return pairs
        return self._merge(
            self.mergeSort(pairs[:m]), 
            self.mergeSort(pairs[m:]),
        )
        