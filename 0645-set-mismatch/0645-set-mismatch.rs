impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0, 0];
        
        for i in 0..n {
            let idx = (nums[i].abs() - 1) as usize;
            if nums[idx] < 0 {
                // Found duplicate
                ans[0] = nums[i].abs();
            } else {
                nums[idx] = -nums[idx];
            }
        }
        
        // Find the missing number (the one that remains positive)
        for i in 0..n {
            if nums[i] > 0 {
                ans[1] = (i + 1) as i32;
                break;
            }
        }
        
        ans
    }
}