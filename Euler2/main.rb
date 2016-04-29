tot = 0
currFib = 1
lastFib = 1
while currFib <= 4000000 do
    newest = currFib + lastFib
    if newest % 2 == 0
        tot += newest
    end
    lastFib = currFib
    currFib = newest
end
puts tot
