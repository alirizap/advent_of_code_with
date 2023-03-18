

def sum_divisors(number):
    
    sum_result = 0
    for i in range(1, number):
        if number % i == 0:
            sum_result += i 

    return sum_result


number = int(input())
if sum_divisors(number) == number:
    print("YES")
else:
    print("NO")