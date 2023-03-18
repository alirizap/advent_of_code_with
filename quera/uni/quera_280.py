

a = int(input())
b = int(input())
c = int(input())

sides = sorted([a, b, c])

if sides[0]**2 + sides[1]**2 == sides[2]**2:
    print("YES")
else:
    print("NO")