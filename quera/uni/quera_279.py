import math

a, x, n = input().split(" ")
a, x, n = int(a), int(x), int(n)

results = []
for k in range(n+1):
    result = math.comb(n, k) * (x**k) * (a**(n-k))
    results.append(result)

print(sum(results))
