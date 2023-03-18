
import math

def quadratic_equation(a, b, c):


    if b == 0 and b == 0:
        return None 

    if a == 0:
        return -c / b 
    
    disc = b**2 - 4*a*c
    
    if disc < 0:
        return None
    elif disc == 0:
        return -b / (2*a)
    else:
        root2 = (-b + math.sqrt(disc)) / (2*a)
        root1 = (-b - math.sqrt(disc)) / (2*a)
        return root1, root2

a = float(input())
b = float(input())
c = float(input())
result = quadratic_equation(a, b, c)

if result == None:
    print("IMPOSSIBLE")

elif isinstance(result, float):
    print("{:.3f}".format(result))

else:
    print("{:.3f}\n{:.3f}".format(result[0], result[1]))