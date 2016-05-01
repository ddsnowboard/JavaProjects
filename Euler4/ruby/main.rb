arr = []
for i in 100...999
  for j in 100...999
    s = (i * j).to_s
    if s.reverse == s
      arr.push(i * j)
    end
  end
end
puts arr.max
