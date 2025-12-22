# Definition for a pair.
# class Pair:
#     def __init__(self, key: int, value: str):
#         self.key = key
#         self.value = value
class Solution:
    def insertionSort(self, pairs: List[Pair]) -> List[List[Pair]]:
        result: List[List[Pair]] = []
        for i in range(len(pairs)):
            j = i
            while j > 0 and pairs[j - 1].key > pairs[j].key:
                tmp = pairs[j - 1]
                pairs[j - 1] = pairs[j]
                pairs[j] = tmp
                j = j - 1
            result.append(pairs.copy())

        return result