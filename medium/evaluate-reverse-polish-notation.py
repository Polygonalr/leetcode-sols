class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for c in tokens:
            if c.isdigit() or len(c) != 1:
                stack.append(int(c))
                continue

            if c == "+":
                stack.append(stack.pop() + stack.pop())
            elif c == "-":
                second = stack.pop()
                stack.append(stack.pop() - second)
            elif c == "*":
                stack.append(stack.pop() * stack.pop())
            elif c == "/":
                second = stack.pop()
                stack.append(int(stack.pop() / second))
        
        return stack[0]
