from math import sqrt
def divisors(i):
	out = []
	for p in range(2, int(sqrt(i)+1)):
		if i % p == 0:
			out.append(p)
			out.append(i/p)
	return out
abundants = []
for i in range(28123):
	if sum(divisors(i)) > i:
		abundants.append(i)
out = []
counter = 0
for i in range(28123):
	counter += 1
	if counter == 100:
		counter = 0
		print(i)
	for p in abundants:
		if p<i:
			if (i-p) in abundants:
				continue
	out.append(i)
print(sum(out))