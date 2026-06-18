# 347 - Top K Frequent Elements

## Description

Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

 

Example 1:

Input: nums = [1,1,1,2,2,3], k = 2

Output: [1,2]

Example 2:

Input: nums = [1], k = 1

Output: [1]

Example 3:

Input: nums = [1,2,1,2,1,2,3,1,3,2], k = 2

Output: [1,2]

 

Constraints:

    1 <= nums.length <= 105
    -104 <= nums[i] <= 104
    k is in the range [1, the number of unique elements in the array].
    It is guaranteed that the answer is unique.

## Solution

To solve this problem, I used a HashMap to count the frequency of each element in the array. Then I collect the entries into a vec of pairs (key, value) and sort them by vakue in descending order. Finally I created a new vec to store the top k elements and I iterate k times over the vec of sorted pairs to store the keys into the final vec.
