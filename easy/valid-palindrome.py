import re

class Solution:
    def isPalindrome(self, s: str) -> bool:
        i = 0
        s = re.sub(r'[\W_]+', '', s)
        s = s.lower()
        while i < len(s)/2:
            if s[i] != s[-i-1]:
                return False
            i += 1
        return True
