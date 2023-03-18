n = int(input())
sentences = []

for _ in range(n):
    s1 = input()
    s2 = input()
    sentences.append((s1, s2))


def check_arrange(first, second):

    index = 0
    res = ""
    for letter in first:
        if index == len(second):
            break
        if letter == second[index]:
            res += letter
            index += 1

    return res == second


for sentence in sentences:
    first = sentence[0]
    second = sentence[1]

    if check_arrange(first, second) or check_arrange(first, second[::-1]):
        print("YES")
    else:
        print("NO")
