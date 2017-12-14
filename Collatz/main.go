package main

import "fmt"

func main() {
	const MAX = 1000000;
	channel := make(chan bool, 100)
	for i := 1; i < MAX; i++ {
		go collatzWorks(i, channel)
	}
	for i := 1; i < MAX; i++ {
		b := <-channel
		if !b {
			fmt.Println("We got a bad one")
		}
	}
}

func collatzWorks(n int, ch chan bool) {
	walker := uint64(n)
	if n == 0 {
		ch <- true
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
			ch <- false
			return
		}
		if walker == newest || walker == secondNewest {
			fmt.Printf("Doesn't work for %d", uint64(n))
			ch <- false
			return
		} else {
			secondNewest = newest
			newest = walker
		}
	}
	ch <- true
}
