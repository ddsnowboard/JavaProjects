package main

import "fmt"

func main() {
	const MAX = 10000000
	for i := 1; i < MAX; i++ {
		if !collatzWorks(i) {
			fmt.Println("We got a bad one")
		}
	}
}

func collatzWorks(n int) bool {
	walker := uint64(n)
	if n == 0 {
		return true
	}
	// These are to make sure that we aren't just going in a circle.
	var newest uint64 = 0
	var secondNewest uint64 = 0
	for walker != 1 {
		if walker%2 == 0 {
			walker /= 2
		} else if walker%2 != 0 {
			walker = 3*walker + 1
		} else {
			fmt.Println("Something really bad just happened...")
			return false
		}
		if walker == newest || walker == secondNewest {
			fmt.Printf("Doesn't work for %d", uint64(n))
			return false
		} else {
			secondNewest = newest
			newest = walker
		}
	}
	return true
}
