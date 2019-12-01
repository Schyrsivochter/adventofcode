#!/usr/bin/python3

summ = 0
while True:
    try:
        i = input()
    except EOFError:
        break
    i = int(i) // 3 - 2
    while i > 0:
        summ += i
        i = int(i) // 3 - 2

print(summ)