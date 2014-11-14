def weekdays():
	days = ["Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday", "Monday"]
	current = 0
	while True:
		yield days[current%7]
		current += 1
weekdays = weekdays()
next_day = lambda: next(weekdays)
days_total = []
# This is the months in order with a function that returns the amount of days in them, 
# with the input to the function being the year. For every month other than February,it doesn't
# do anything, but in February it tells whether the year is a leap year. 
months = [lambda x: 31, lambda x: 29 if (x % 4 == 0 and x % 100 != 0) or (x % 400 == 0) else 28, lambda x: 31, lambda x:30, lambda x: 31, lambda x : 30, lambda x: 31, lambda x: 31, lambda x: 30, lambda x: 31, lambda x: 30, lambda x: 31]
for i in range(1901, 2001):
	for j in months:
		for p in range(j(i)):
			if p == 0:
				days_total.append([next_day(), True])
			else:
				days_total.append([next_day(), False])
number = 0
for i in days_total:
	if i[0] == "Sunday" and i[1]:
		number += 1
print(number)