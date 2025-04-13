from pathlib import Path
from pprint import pprint
from enum import Enum, auto
import json
import re
import sys

DIRENAME_REGEX = re.compile(r"([a-z]+) iterate (\d+)")

# run with the criterion directory as argument

class Iteration:
    def __init__(self, type, count, time_nanos):
        self.type = type
        self.count = count
        self.time_nanos = time_nanos

    def __repr__(self):
        return "Iteration(type = {}, count = {}, time_nanos = {}".format(self.type, self.count, self.time_nanos)

    def __str__(self):
        return repr(self)

criterion_dir = Path(sys.argv[1])
def parse_dir(dirpath):
    m = DIRENAME_REGEX.search(dirpath.name)
    if not m:
        return None
    else:
        which = m.group(1)
        count = int(m.group(2))
        estimate_file = dirpath / "new" / "estimates.json"
        with estimate_file.open() as f:
            json_obj = json.load(f)
            time_nanos = int(json_obj["mean"]["point_estimate"])
        return Iteration(type = which, count = count, time_nanos = time_nanos)

runs = map(parse_dir, criterion_dir.iterdir())
for r in runs:
    print("{},{},{}".format(r.type, r.count, r.time_nanos))
