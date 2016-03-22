with open('input.txt', 'r') as f:
    grid = [i.strip("\n").split(" ") for i in f]
    for i in range(len(grid)):
        grid[i] = [int(i) for i in grid[i]]
    biggest = 0
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i][j + 1] * grid[i][j + 2] * grid[i][j + 3]>biggest):
                    biggest = grid[i][j] * grid[i][j + 1] * grid[i][j + 2] * grid[i][j + 3]
            except IndexError:
                pass
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i + 1][j] * grid[i + 2][j] * grid[i + 3][j]>biggest):
                    biggest = grid[i][j] * grid[i + 1][j] * grid[i + 2][j] * grid[i + 3][j]
            except IndexError:
                pass
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i + 1][j + 1] * grid[i + 2][j + 2] * grid[i + 3][j + 3]>biggest):
                    biggest = grid[i][j] * grid[i + 1][j + 1] * grid[i + 2][j + 2] * grid[i + 3][j + 3]
            except IndexError:
                pass
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i + 1][j-1] * grid[i + 2][j-2] * grid[i + 3][j-3]>biggest):
                    biggest = grid[i][j] * grid[i + 1][j-1] * grid[i + 2][j-2] * grid[i + 3][j-3]
            except IndexError:
                pass
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i-1][j-1] * grid[i-2][j-2] * grid[i-3][j-3]>biggest):
                    biggest = grid[i][j] * grid[i-1][j-1] * grid[i-2][j-2] * grid[i-3][j-3]
            except IndexError:
                pass
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            try:
                if(grid[i][j] * grid[i-1][j + 1] * grid[i-2][j + 2] * grid[i-3][j + 3]>biggest):
                    biggest = grid[i][j] * grid[i-1][j + 1] * grid[i-2][j + 2] * grid[i-3][j + 3]
            except IndexError:
                pass
    with open('out.txt', 'w') as w:
        w.write(str(biggest))
