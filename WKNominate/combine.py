import csv
from sys import stdout


decomposition_fieldnames = ["component_1", "component_2", "component_3", "component_4", "group_n"]
member_fieldnames = ['name','icpsr','party']
dict_readers = [
    csv.DictReader(open("decompositions.csv", newline=""), fieldnames=decomposition_fieldnames),
    csv.DictReader(open("ordered_member_data.csv"))
]

rows = (dict(**l, **r) for (l, r) in zip(*dict_readers))
writer = csv.DictWriter(stdout, decomposition_fieldnames + member_fieldnames)
writer.writeheader()
for row in rows:
    writer.writerow(row)
