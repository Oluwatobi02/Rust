def addFun(n):
    if n<=0:
        return 0
    elif n ==1:
        return 2
    return addFun(n-1) + addFun(n-2)

print(addFun(6))