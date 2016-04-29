TOP = 1000
output = (1...TOP).select {|i| i % 3 == 0 or i % 5 == 0}.reduce(:+)
puts output
