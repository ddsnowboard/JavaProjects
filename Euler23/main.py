from math import sqrt
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
		abundants[float(i)] = True
out = []
counter = 0
found = False
for i in range(28123):
	found = False
	if counter == 100:
		counter = 0
		print(i)
	counter += 1
	for p in abundants.keys():
		if p<i:
			try:
				if abundants[i-p]:
					found = True
					break
			except KeyError:
				pass
	if not found:
		out.append(i) 
		
print(sorted(out))
with open("output.txt", 'w') as w:
	w.write(str(sum(out)))