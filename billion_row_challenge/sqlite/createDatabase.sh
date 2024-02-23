# Create the database first
sed "s/'/''/g" ../medium_measurements.txt| awk -F ';' -f inserter.awk | sqlite3 db.sqlite
