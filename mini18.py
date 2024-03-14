# op: (priority, right-to-left)
OPS = {
    "!": (0, True),
    "~": (0, True),
    "**": (1, True),
    "*": (2, False),
    "/": (2, False),
    "%": (2, False),
    "+": (3, False),
    "-": (3, False),
    "==": (5, False),
    "!=": (5, False),
    "&": (6, False),
    "^": (7, False),
    "|": (8, False),
    "&&": (9, False),
    "||": (10, False),
}

if __name__ == "__main__":
    expr = input().split()
    expr.reverse()

    output = []
    stack = []

    while expr:
        tok = expr.pop()
        if tok == '(':
            stack.append(None)
        elif tok == ')':
            while stack[-1]:
                output.append(stack.pop())
        elif tok in OPS:
            op, (pr, righta) = tok, OPS[tok]
            while stack and (OPS[stack[-1]][0], righta) <= (pr, False):
                output.append(stack.pop())
            stack.append(op)
        elif tok.isalnum():
            output.append(tok)
        else:
            raise ValueError(f"Unexpected '{tok}'")

    output += stack[::-1]
    print(" ".join(output))
