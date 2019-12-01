#!/usr/bin/python3
import sys

summ = 0
for line in sys.stdin.readlines():
    sum += int(line) // 3 - 2

print(summ)
