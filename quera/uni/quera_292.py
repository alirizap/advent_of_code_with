

def show(number):
    n = f"{number:.4f}"
    return float(n[:-1])

n = int(input())

numbers = []
for i in range(n):
    numbers.append(float(input()))

print("Max: {:.3f}".format(show(max(numbers))))
print("Min: {:.3f}".format(show(min(numbers))))
print("Avg: {:.3f}".format(sum(numbers)/n))