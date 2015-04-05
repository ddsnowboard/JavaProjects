with open("B-large-practice.in") as f:
    with open("output.txt", 'w') as w:
        f.readline()
        for i, j in enumerate(f):
            j = j.strip()
            w.write("Case #{}: {}\n".format(i+1, " ".join(j.split(" ")[::-1])))