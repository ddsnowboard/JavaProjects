import sqlite3

conn = sqlite3.connect("speedrundata.sqlite")
conn.row_factory = sqlite3.Row
