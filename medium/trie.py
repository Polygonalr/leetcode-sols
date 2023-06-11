'''
Simple trie tree implementation in Python. Runtime and memory can be further optimized by using a dictionary instead of
a 26-element list to store child nodes.
'''

def cti(c):
    return ord(c) - ord('a')

class Node:
    def __init__(self):
        self.next = [None for _ in range(26)]
        self.end = False
    
    def add_next(self, s):
        if len(s) == 0:
            self.end = True
            return
        if self.next[cti(s[0])] is None:
            self.next[cti(s[0])] = Node()
        self.next[cti(s[0])].add_next(s[1:])

class Trie:
    def __init__(self):
        self.root = Node()

    def insert(self, word: str) -> None:
        self.root.add_next(word)

    def search(self, word: str) -> bool:
        curr = self.root
        for c in word:
            if curr.next[cti(c)] is None:
                return False
            curr = curr.next[cti(c)]
        return curr.end

    def startsWith(self, prefix: str) -> bool:
        curr = self.root
        for c in prefix:
            if curr.next[cti(c)] is None:
                return False
            curr = curr.next[cti(c)]
        return True
