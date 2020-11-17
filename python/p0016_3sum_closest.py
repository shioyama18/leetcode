from typing import List


class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()
        closest = float('inf')
        n = len(nums)

        for i in range(n-2):
            l, r = i + 1, n - 1
            local_closest = float('inf')

            while l < r:
                total = n[i] + n[l] + n[r]
                if abs(target - total) < abs(target - local_closest):
                    local_closest = target

                if total < target:
                    l += 1
                elif total > target:
                    r -= 1
                else:
                    return total
            
            if abs(target - local_closest) < abs(target - closest):
                closest = local_closest

        return closest
