# 271 - Encode and Decode Strings

## Description

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

Machine 1 (sender) has the function:

String encode(List<String> strs) {
    // ... your code
    return encoded_string;
}

Machine 2 (receiver) has the function:

List<String> decode(String encoded_string) {
    // ... your code
    return decoded_strs;
}

So Machine 1 does:

String encoded_string = encode(strs);

and Machine 2 does:

List<String> decoded_strs = decode(encoded_string);

decoded_strs in Machine 2 should be the same as the input strs in Machine 1.

Implement the encode and decode methods.

Example 1:

Input: strs = ["Hello","World"]

Output: ["Hello","World"]

Explanation:

Solution solution = new Solution();
String encoded_string = solution.encode(strs);

// Machine 1 ---encoded_string---> Machine 2

List<String> decoded_strs = solution.decode(encoded_string);


Example 2:

Input: strs = [""]

Output: [""]


Constraints:

    0 <= strs.length < 100
    0 <= strs[i].length < 200
    strs[i] contains any possible characters out of 256 valid ASCII characters.

## Solution

To solve this problem I decided to encode each word with this format: "word_length#word" first we put the word length, then a '#' as splitter and then the word it self.

### Encode method:

To implement the defined encoding in the encode method I just iterate over the array of Strings and add to a String the length and the word with the '#' splitter of each word.

### Decode method:

To implement the defined encoding in the decode method I created this vars:
  - An array to store the words.
  - A index to iterate the input string.
  - A string to append the digits of the word length.

Once the vars are defined, I iterate over the input string using the index to get the current character and checking if it is '#'. In the case it's true, I parse the digits of the word length to an interger and use this length to get the word and move the index to the next word. In the case it's false, I added the digit to the word length digits and increase the index by 1.
