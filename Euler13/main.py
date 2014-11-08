end = 0
with open("input.txt", 'r') as f:
	for i in f:
		end+=int(i)
with open('output.txt', 'w') as w:
	w.write(str(end)[:10])