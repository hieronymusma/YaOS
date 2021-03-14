#!/bin/python3

import sys

virtual = int(sys.argv[1], 0)

current = virtual >> 12

p1 = current & 0x1ff
current = current >> 9

p2 = current & 0x1ff
current = current >> 9

p3 = current & 0x1ff
current = current >> 9

p4 = current & 0x1ff

print(f"P4: {p4}; P3: {p3}; P2: {p2}; P1: {p1};")