#!/usr/bin/python3

summ = 0
while True:
    try:
        i = input()
    except EOFError:
        break
    sum += int(i) // 3 - 2

print(summ)
