use std::collections::HashMap;


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

/*
    1672. Richest Customer Wealth

    You are given an m x n integer grid accounts where accounts[i][j] 
    is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the ​​​​​​​​​​​th​​​​e bank. 
    Return the wealth that the richest customer has.

    A customer's wealth is the amount of money they have in all their bank accounts. 
    The richest customer is the customer that has the maximum wealth.

    Example 1:

    Input: accounts = [[1,2,3],[3,2,1]]
    Output: 6
    Explanation:
    1st customer has wealth = 1 + 2 + 3 = 6
    2nd customer has wealth = 3 + 2 + 1 = 6
    Both customers are considered the richest with a wealth of 6 each, so return 6.

    T: 12.18
*/

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    let mut sum = 0;
    for i in accounts {
        for j in i {
            sum += j;
        }
        if max < sum {
            max = sum;
        }
        sum = 0;
    }
    max
}