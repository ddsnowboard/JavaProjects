import csv
from sys import stdout

def map_party_code(code):
    if int(code) == 200:
        return "republican"
    elif int(code) == 100:
        return "democrat"
    else:
        return 'other'

with open("votes.csv", newline="") as f:
    reader = list(csv.DictReader(f))
    all_member_numbers_in_dataset = sorted({int(r["icpsr"]) for r in reader})
    with open("members.csv", newline="") as m:
        members_reader = list(csv.DictReader(m))
        output = csv.writer(stdout)
        output.writerow(["name", "icpsr", "party"])
        for member_id in all_member_numbers_in_dataset:
            try:
                member = next(m for m in members_reader if int(m["icpsr"]) == member_id)
            except StopIteration:
                print("Member id was {}".format(member_id))
                raise
            row = [member["bioname"], member["icpsr"], map_party_code(member["party_code"])]
            output.writerow(row)
