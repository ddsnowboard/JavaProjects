def is_triple(a, b, c)
    return a ** 2 + b ** 2 == c ** 2
end

for a in 1...1000
    for b in a...1000
        c = 1000 - (a + b)
        if is_triple(a, b, c)
            puts a * b * c
        end
    end
end
