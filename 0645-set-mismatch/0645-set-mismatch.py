class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        n = len(nums)
        ans = [0, 0]
        
        # Mark visited numbers by making them negative
        for num in nums:
            idx = abs(num) - 1
            if nums[idx] < 0:
                # Found duplicate
                ans[0] = abs(num)
            else:
                nums[idx] = -nums[idx]
        
        # Find the missing number (the one that remains positive)
        for i in range(n):
            if nums[i] > 0:
                ans[1] = i + 1
                break
        
        return ans