import sqlite3

conn = sqlite3.connect("data.sqlite")
conn.row_factory = sqlite3.Row
