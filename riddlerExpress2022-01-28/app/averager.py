from statistics import mean
"""
Splits the times.csv file (which shows how long each call to is_alive takes in ns) into chunks of a set size and prints the average of each chunk.
Lets me see if the times are increasing with increasing generations.
They seem not to be, meaning that a smaller HashSet won't really help me.
"""

def chunk_averages(i, size):
    iterable = iter(i)
    counter = 0
    s = 0
    while True:
        while counter < size:
            try:
                s += int(next(iterable))
                counter += 1
            except StopIteration:
                yield s / counter
                return
        yield s / counter
        counter = 0
        s = 0

with open("times.csv") as f:
    for chunk in chunk_averages(f, 10000):
        print(chunk)
