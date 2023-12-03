with open("src/day_1/input.txt", "r") as f:
    input = f.read().split("\n")


# Part 1
def get_first_digit(line: str) -> int:
    for c in line:
        if c.isdigit():
            return int(c)


def get_last_digit(line: str) -> int:
    for c in line[::-1]:
        if c.isdigit():
            return int(c)


sum = 0
for line in input:
    first_digit = get_first_digit(line)
    last_digit = get_last_digit(line)
    sum += first_digit * 10 + last_digit

print(sum)


# Part 2
def get_first_spelt_digit(line: str) -> int:
    for i, c in enumerate(line):
        if c.isdigit():
            return int(c)
        else:
            for d, val in enumerate(
                ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            ):
                if line[i:].startswith(val):
                    return d + 1


def get_last_spelt_digit(line: str) -> int:
    # Loop from the end of the string
    for i, c in reversed(list(enumerate(line))):
        if c.isdigit():
            return int(c)
        else:
            for d, val in enumerate(
                ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            ):
                if line[i:].startswith(val):
                    return d + 1


sum = 0
for line in input:
    first_digit = get_first_spelt_digit(line)
    last_digit = get_last_spelt_digit(line)
    sum += first_digit * 10 + last_digit
print(sum)
