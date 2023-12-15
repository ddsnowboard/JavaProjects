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

years_of_the_goat = [(parse_date(l).date(), parse_date(r).date()) for (l, r) in [
('13 February 1907', 	'01 February 1908'),
('01 February 1919', 	'19 February 1920'),
('17 February 1931', 	'05 February 1932'),
('05 February 1943', 	'24 January 1944'),
('24 January 1955', 	'11 February 1956'),
('09 February 1967', 	'29 January 1968'),
('28 January 1979', 	'15 February 1980'),
('15 February 1991', 	'03 February 1992'),
('01 February 2003', 	'21 January 2004'),
('19 February 2015', 	'07 February 2016'),
('06 February 2027', 	'25 January 2028'),
('24 January 2039', 	'11 February 2040'),
('11 February 2051', 	'31 January 2052'),
('29 January 2063', 	'16 February 2064'),
('15 February 2075', 	'04 February 2076'),
('03 February 2087', 	'23 January 2088'),
('21 January 2099', 	'08 February 2100')
]]

def is_cancer(d):
    start = datetime.date(year=d.year, month=6, day=21)
    end = datetime.date(year=d.year, month=7, day=22)
    return d >= start and d <= end

def is_libra(d):
    start = datetime.date(year=d.year, month=9, day=23)
    end = datetime.date(year=d.year, month=10, day=23)
    return d >= start and d <= end

latitude = 40.73007
longitude = -73.74856

def distance(lat, long):
    return ((lat - latitude) ** 2 + (long - longitude) ** 2) ** (1/2)

c = common.conn.cursor()

c.execute("select * from customers")
data = ((r["name"], r["phone"], datetime.date.fromisoformat(r["birthdate"]), r["lat"], r["long"]) for r in c)
rabbits = ((name, phone, bd, lat, long) for (name, phone, bd, lat, long) in data if any(bd >= start and bd <= end for (start, end) in years_of_the_goat))
cancers = ((name, phone, bd, lat, long) for (name, phone, bd, lat, long) in rabbits if is_libra(bd))
ordered = sorted(cancers, key=lambda tup: distance(tup[-2], tup[-1]))
print(list(ordered)[0])
