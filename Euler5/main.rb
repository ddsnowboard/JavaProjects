def is_divisible(n)
    numbers = [20, 19, 18, 17, 16, 15, 14, 13, 12, 11]
    for i in numbers
        if n % i != 0
            return false
        end
    end
    return true
end

i = 20
while not is_divisible(i)
    i += 20
end
puts i
