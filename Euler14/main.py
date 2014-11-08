times_list = {}
def collatz(start):
	global times_list
	times = 0
	current = start
	while not current == 1:
		times+=1
		if current % 2 == 0:
			current = current / 2
		else:
			current = 3*current + 1
		try:
			return times + times_list[current]
		except KeyError:
			pass
	return times
counter = 0
for i in range(1,1000000,2):
	times_list[i] = collatz(i)
	counter += 1
	if counter == 5000:
		counter = 0
		print(i)
print(max(times_list.items(), key= lambda x: x[1])[0])