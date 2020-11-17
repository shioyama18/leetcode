class Solution:
    def romanToInt(self, s: str) -> int:
        romans = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000,
        }
        output = romans.get(s[-1])

        for i in reversed(range(len(s)-1)):
            if romans[s[i]] < romans[s[i+1]]:
                output -= romans[s[i]]
            else:
                output += romans[s[i]]

        return output
