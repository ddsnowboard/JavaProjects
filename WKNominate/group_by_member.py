import csv
from sys import argv, stdout

with open(argv[1], newline="") as f:
    reader = list(csv.DictReader(f))
    all_members = sorted({int(r["icpsr"]) for r in reader})
    all_votes = sorted({int(r["rollnumber"]) for r in reader})
    vote_code_mapping = {
        1: 1.0,
        2: 1.0,
        3: 1.0,
        4: 0.0,
        5: 0.0,
        6: 0.0
    }
    by_roll_and_member = {(int(r["icpsr"]), int(r["rollnumber"])): vote_code_mapping.get(int(r["cast_code"]), 0.5) for r in reader}
    writer = csv.writer(stdout)
    for member in all_members:
        row = [by_roll_and_member.get((member, rollnumber), 0.5) for rollnumber in all_votes]
        writer.writerow(row)
