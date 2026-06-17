# 242 - Valid Anagram

## Description

Given two strings s and t, return true if t is an of s, and false otherwise.

 

Example 1:

Input: s = "anagram", t = "nagaram"

Output: true

Example 2:

Input: s = "rat", t = "car"

Output: false

 

Constraints:

    1 <= s.length, t.length <= 5 * 104
    s and t consist of lowercase English letters.

## Solution

To solve this problem, we need to know if the two strings have the same characters with the same frequency. To do this, I used a HashMap for each string and added each character as a key and its frequency as the value. Then I compared the two HashMaps. If they are equal, then the two strings are anagrams of each other.
