from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        mapping = dict()
        for i in range(len(nums)):
            complement = target - nums[i]

            if complement in mapping:
                return [mapping[complement], i]

            mapping[nums[i]] = i
