class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        start = longest = 0
        found = dict()

        for i in range(len(s)):
            if s[i] in found:
                start = max(start, found[s[i]])
            longest = max(longest, i - start + 1)
            found[s[i]] = i + 1

        return longest
