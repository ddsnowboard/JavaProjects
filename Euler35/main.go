package main

import "fmt"
import "math"

func main() {
	const TOP = 1000000
	primes := make(map[int]bool)
	for i := 2; i <= TOP; i++ {
		primes[i] = true
	}
	for divisor := 2; divisor <= TOP; divisor++ {
		if primes[divisor] {
			removeMultiples(divisor, &primes)
		}
	}
	fmt.Printf("There are %d primes below a million\n", countTrues(&primes))

	count := 0
	for i := 2; i <= TOP; i++ {
		num := i
		nRotations := numberLength(i)
		isCircular := true
		for n := 0; n < nRotations; n++ {
			if !primes[num] {
				isCircular = false
				break
			}
			num = rotate(num)
		}
		if isCircular {
			count++
		}
	}
	fmt.Printf("There are %d circular primes below %d\n", count, TOP)
}

func removeMultiples(d int, dict *map[int]bool) {
	walker := d + d
	for {
		_, in := (*dict)[walker]
		if in {
			(*dict)[walker] = false
		} else {
			break
		}
		walker += d
	}
}

func countTrues(dict *map[int]bool) int {
	out := 0
	for _, v := range *dict {
		if v == true {
			out++
		}
	}
	return out
}

func numberLength(n int) int {
	return int(math.Log10(float64(n))) + 1
}

func rotate(n int) int {
    length := numberLength(n)
    lastDigit := n % 10
    rest := n / 10
    return rest + lastDigit * int(math.Pow(float64(10), float64(length - 1)))
}
