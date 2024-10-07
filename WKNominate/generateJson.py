import csv
from sys import argv, stdin
import json

reader = csv.DictReader(stdin)
print("let members = ")
print(json.dumps(list(reader)))
