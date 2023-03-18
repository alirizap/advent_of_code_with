# God Bless CHAT GPT !!!


def find_numbers(digits_length, target):

    if target == 0:
        if digits_length == 1:
            print(0, 0)
        else:
            print(-1, -1)
        return

    if target > 9 * digits_length:
        print(-1, -1)
        return

    smallest_str = ""
    target_copy = target
    for i in range(digits_length):
        if i == 0:
            d = max(1, target_copy - 9 * (digits_length - 1))
        else:
            d = max(0, target_copy - 9 * (digits_length - i - 1))

        smallest_str += str(d)
        target_copy -= d

    smallest = int(smallest_str)

    largest_str = ""
    target_copy = target
    for i in range(digits_length):
        d = min(9, target_copy)
        target_copy -= d
        largest_str += str(d)
    largest = int(largest_str)

    print(smallest, largest)


line = input()
m, s = map(int, line.split(" "))
find_numbers(m, s)
