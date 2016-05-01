def quad(a, b, c)
  return [(-b + (b ** 2 - 4 * a * c) ** 0.5) / (2 * a), (-b - (b ** 2 - 4 * a * c) ** 0.5) / (2 * a)]
end

class Pentagoner
  @@memo = Hash.new
  def initialize
    @index = 0
  end
  def next_pentagon
    out = pentagon(@index)
    @index += 1
    return out
  end
  def pentagon(n)
    out = @@memo[n]
    if not out
      @@memo[n] = n * (3 * n - 1) / 2
      return @@memo[n]
    else
      return out
    end
  end
  def is_pentagon(n)
    result = quad(3, -1, -2 * n)
    out = result.reduce(false) {|acc, i| acc or (i % 1 == 0 and i >= 1)}
    if out
      for i in result.select {|i| i >= 1 and i % 1 == 0}
        @@memo[i] = n
      end
    end
    return out
  end
end

penta = Pentagoner.new
for j in 1...10000
  for k in 1...10000
    pJ = penta.pentagon(j)
    pK = penta.pentagon(k)
    if penta.is_pentagon(pJ + pK) and penta.is_pentagon((pJ - pK).abs)
      puts (pJ - pK).abs
    end
  end
end
