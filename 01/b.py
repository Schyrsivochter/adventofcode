#!/usr/bin/python3
import sys

summ = 0
for line in sys.stdin:
    i = int(line) // 3 - 2
    while i > 0:
        summ += i
        i = int(i) // 3 - 2

print(summ)
