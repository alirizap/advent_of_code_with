
a = int(input())
b = int(input())

while b != 1 :
    tmp = a 
    a = b - a 
    b = tmp 
    print(b)

print(a)