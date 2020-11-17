class Solution:
    def myAtoi(self, str: str) -> int:
        str = str.lstrip()
        if len(str) == 0:
            return 0

        sign = ''
        if str[0] == '-' or str[0] == '+':
            sign = str[0]
            str = str[1:]

        res = 0
        for i in range(len(str)):
            if not str[i].isdigit():
                break
            res = res * 10 + int(str[i])

        if sign == '-':
            res = -res
            
        if res < -2 ** 31:
            return -2 ** 31
        elif res > 2 ** 31 - 1:
            return 2 ** 31 - 1
        else:
            return res
