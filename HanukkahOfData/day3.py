import datetime
import common

months = {m: idx + 1 for (idx, m) in enumerate("January February March April May June July August September October November December".split(" "))}
def parse_date(s):
    return datetime.datetime.strptime(s, "%d %B %Y")


years_of_the_rabbit = [(parse_date(l).date(), parse_date(r).date()) for (l, r) in [
("29 January 1903", "15 February 1904"),
("14 February 1915", "03 February 1916"),
("02 February 1927", "22 January 1928"),
("19 February 1939", "07 February 1940"),
("06 February 1951", "26 January 1952"),
("25 January 1963", "12 February 1964"),
("11 February 1975", "30 January 1976"),
("29 January 1987", "16 February 1988"),
("16 February 1999", "04 February 2000"),
("03 February 2011", "22 January 2012"),
("22 January 2023", "09 February 2024"),
("08 February 2035", "27 January 2036"),
("26 January 2047", "13 February 2048"),
("11 February 2059", "01 February 2060"),
("31 January 2071", "18 February 2072"),
("17 February 2083", "05 February 2084"),
("05 February 2095", "24 January 2096")
        ]]

def is_cancer(d):
    start = datetime.date(year=d.year, month=6, day=21)
    end = datetime.date(year=d.year, month=7, day=22)
    return d >= start and d <= end

latitude = 40.70895
longitude = -73.80856

def distance(lat, long):
    return ((lat - latitude) ** 2 + (long - longitude) ** 2) ** (1/2)

c = common.conn.cursor()

c.execute("select * from customers")
data = ((r["name"], r["phone"], datetime.date.fromisoformat(r["birthdate"]), r["lat"], r["long"]) for r in c)
rabbits = ((name, phone, bd, lat, long) for (name, phone, bd, lat, long) in data if any(bd >= start and bd <= end for (start, end) in years_of_the_rabbit))
cancers = ((name, phone, bd, lat, long) for (name, phone, bd, lat, long) in rabbits if is_cancer(bd))
ordered = sorted(cancers, key=lambda tup: distance(tup[-2], tup[-1]))
print(list(ordered))
