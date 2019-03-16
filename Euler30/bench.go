package main

import "math"

type NumType = int64

type RetVal struct {
	value  NumType
	digits []NumType
}

func main() {
	first := make(chan *RetVal)
	last := make(chan []NumType)
	go checkEquals(first, last)
        go consume(last)
        r := RetVal{value: 123456, digits: []NumType{1,2,3,4,5,6}}
        for p := 2; p  < 1000000; p++ {
		first <- &r
	}
	close(first)
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

func consume(c chan []NumType) {
    for _ = range c {
        // Do nothing
    }
}
