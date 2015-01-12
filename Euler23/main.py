from math import sqrt
def binarySearch(l, i):
	if len(l) == 1:
		if l == [i]:
			return True
		else:
			return False
	m = l[int(len(l)/2)]
	if m == i:
		return True
	elif m > i:
		return binarySearch(l[:int(len(l)/2)], i)
	elif m < i:
		return binarySearch(l[int(len(l)/2):], i)
	return False
def divisors(i):
	out = []
	for p in range(2, int(sqrt(i)+1)):
		if i % p == 0:
			out.append(p)
			out.append(i/p)
	return out
abundants = {}
for i in range(28123):
	if sum(divisors(i)) > i:
		abundants[i] = True
out = []
counter = 0
for i in range(28123):
	if counter == 100:
		counter = 0
		print(i)
	counter += 1
	for p in abundants:
		if p<i:
			try:
				if abundants[i-p]:
					continue
			except KeyError:
				pass
	out.append(i)
print(sum(out))