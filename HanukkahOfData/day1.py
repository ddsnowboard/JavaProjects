import common
from functools import reduce

def count(s, c):
    return sum(1 for cc in s if cc == c)

def get_last_name(name):
    names = name.split(" ")
    whole_last_name = names[1:]
    return " ".join(whole_last_name)

number_mapping = {
    2: "ABC",
    3: "DEF",
    4: "GHI",
    5: "JKL",
    6: "MNO",
    7: "PQRS",
    8: "TUV",
    9: "WXYZ"
}

letter_mapping = reduce(lambda x, y: dict(**x, **y), ({c: str(n) for c in s} for n, s in number_mapping.items()))
def build_phone_number(name):
    return "".join(letter_mapping[c.upper()] for c in name)

with common.conn:
    c = common.conn.cursor()
    c.execute("SELECT name, phone from customers")
    names_and_numbers = ((s["name"], s["phone"].replace("-", '')) for s in c)
    last_names = ((get_last_name(name), phone)  for (name, phone) in names_and_numbers)
    right_length = ((n, p) for (n, p) in last_names if len(n) == 10 and " " not in n)
    with_name_numbers = ((n, p, build_phone_number(n)) for (n, p) in right_length)
    answer = ((n, p, pp) for (n, p, pp) in with_name_numbers if p == pp)
    print(list(answer))
