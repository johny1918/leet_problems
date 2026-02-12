


/*
1480. Running Sum of 1d Array

Given an array nums. We define a running sum of an array as 
runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.

Example 1:

Input: nums = [1,2,3,4]
Output: [1,3,6,10]
Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].

T: 3.55
*/

pub fn sum_one_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ret_val: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut index = 0;
    for n in nums {
        sum += n;
        ret_val.insert(index, sum);
        index += 1;
    }
    ret_val
}