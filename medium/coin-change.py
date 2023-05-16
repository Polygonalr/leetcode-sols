class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        self.memo = [-1] * (amount + 1)
        self.memo[0] = 0
        for coin in coins:
            if coin <= amount:
                self.memo[coin] = 1
        
        for i in range(amount):
            for c in coins:
                if i+c > amount or self.memo[i] == -1:
                    continue
                if self.memo[i+c] == -1:
                    self.memo[i+c] = self.memo[i] + 1
                self.memo[i+c] = min(self.memo[i+c], self.memo[i] + 1)
        
        return self.memo[amount]
        
