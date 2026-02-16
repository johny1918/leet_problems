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

/*
    412. Fizz Buzz

    Given an integer n, return a string array answer (1-indexed) where:

    answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
    answer[i] == "Fizz" if i is divisible by 3.
    answer[i] == "Buzz" if i is divisible by 5.
    answer[i] == i (as a string) if none of the above conditions are true.
    

    Example 1:
    Input: n = 3
    Output: ["1","2","Fizz"]

    Example 2:
    Input: n = 5
    Output: ["1","2","Fizz","4","Buzz"]

    T: 15.33
*/


pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut answer = vec![];

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0  {
            answer.push("FizzBuzz".to_string());
        }
        else if i % 3 == 0 {
            answer.push("Fizz".to_string());
        }
        else if i % 5 == 0 {
            answer.push("Buzz".to_string());
        }
        else {
            answer.push(i.to_string());
        }
    }

    answer
}


/*

    Given an integer num, return the number of steps to reduce it to zero.

    In one step, if the current number is even, you have to divide it by 2, 
    otherwise, you have to subtract 1 from it.

    

    Example 1:

    Input: num = 14
    Output: 6
    Explanation: 
    Step 1) 14 is even; divide by 2 and obtain 7. 
    Step 2) 7 is odd; subtract 1 and obtain 6.
    Step 3) 6 is even; divide by 2 and obtain 3. 
    Step 4) 3 is odd; subtract 1 and obtain 2. 
    Step 5) 2 is even; divide by 2 and obtain 1. 
    Step 6) 1 is odd; subtract 1 and obtain 0.

    T: 8:43
*/
pub fn number_of_steps(mut num: i32) -> i32 {
    let mut steps = 0;
    loop {
        if num != 0 {
            if num % 2 == 0 {
                num = num / 2;

            }
            else {
                num -= 1;
            }
            steps += 1;
        }
        
        if num == 0 {
            break;
        }
    }
    steps
}