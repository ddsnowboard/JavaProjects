TOP = 100
puts (1..TOP).reduce(0) {|acc, i| acc + i * i} - (1..TOP).reduce(:+) ** 2
