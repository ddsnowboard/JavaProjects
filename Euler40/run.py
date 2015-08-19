#!/usr/bin/python3 
s = ""
count = 1
while (len(s)) <= 1000000:
    s += str(count)
    count += 1
out = 1
for i in (1, 10, 100, 1000, 10000, 100000, 1000000):
    print(s[i - 1])
    out *= int(s[i - 1])
print(out)
