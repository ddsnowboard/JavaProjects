def weekdays():
	days = ["Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday", "Monday"]
	current = 0
	while True:
		yield days[current%7]
		current += 1
weekdays = weekdays()
next_day = lambda: next(weekdays)
days_total = []
months = [lambda x: 31, lambda x: 29 if (x % 4 == 0 and x % 100 != 0) or (x % 400 == 0) else 28, lambda x: 31, lambda x:30, lambda x: 31, lambda x : 30, lambda x: 31, lambda x: 31, lambda x: 30, lambda x: 31, lambda x: 30, lambda x: 31]
for i in range(1901, 2000):
	for j in months:
		for p in range(j(i)):
			days_total.append(next_day())
number = 0
for i in days_total:
	if i == "Sunday":
		number += 1
print(number)
print([i for i in range(1900, 2001) if months[1](i) == 29])