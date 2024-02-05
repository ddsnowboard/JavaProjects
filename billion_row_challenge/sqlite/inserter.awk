{ 
    if (NR % 100000 == 0) {
        print "COMMIT;"
    }
    if(NR % 100000 == 0 || NR == 1) {
        print "BEGIN TRANSACTION;"
    }
    printf "INSERT INTO cities VALUES ('%s', %s);\n", $1, $2 
}
END { print "COMMIT;" }
