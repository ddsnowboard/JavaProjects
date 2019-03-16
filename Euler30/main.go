package main

import "fmt"
import "math"

type NumType = int64

type RetVal struct {
	value  NumType
	digits []NumType
}

func main() {
	first := make(chan NumType)
	middle := make(chan *RetVal)
	last := make(chan []NumType)
	go splitDigits(first, middle)
	go checkEquals(middle, last)
	for i := NumType(2); i < 1000000; i++ {
		first <- i
	}
	close(first)
	ret := <-last
	fmt.Println("Sum was", sum(ret))
}

func splitDigits(in chan NumType, out chan *RetVal) {
	for num := range in {
		digits := make([]NumType, 0)
		input := num
		for input > 0 {
			ones := input % 10
			input /= 10
			digits = append(digits, ones)
		}
		out <- &RetVal{value: num, digits: digits}
	}
	close(out)
}

func checkEquals(in chan *RetVal, out chan []NumType) {
	const POWER float64 = 5
	collection := make([]NumType, 0)
	for val := range in {
		sum := NumType(0)
		for _, n := range val.digits {
			sum += NumType(math.Pow(float64(n), POWER))
		}
		if sum == val.value {
			collection = append(collection, sum)
		}
	}
	out <- collection
	close(out)
}

func sum(arr []NumType) NumType {
	out := NumType(0)
	for _, i := range arr {
		out += i
	}
	return out
}

func mergeNumType(a chan []NumType, b chan []NumType, out chan []NumType) {
	for a != nil && b != nil {
		select {
		case n, ok = <-a:
			if ok {
				out <- n
			} else {
				a = nil
			}
		case n, ok = <-b:
			if ok {
				out <- n
			} else {
				b = nil
			}
		}
	}
	var rem chan []NumType
	if a == nil {
		rem = b
	} else {
		rem = a
	}
	for n := range rem {
		out <- n
	}
	close(out)
}

func mergeRetVal(a chan []*RetVal, b chan []*RetVal, out chan []*RetVal) {
	for a != nil && b != nil {
		select {
		case n, ok = <-a:
			if ok {
				out <- n
			} else {
				a = nil
			}
		case n, ok = <-b:
			if ok {
				out <- n
			} else {
				b = nil
			}
		}
	}
	var rem chan []*RetVal
	if a == nil {
		rem = b
	} else {
		rem = a
	}
	for n := range rem {
		out <- n
	}
	close(out)
}
