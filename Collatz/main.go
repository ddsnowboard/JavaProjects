package main

import "fmt"
import "sync"

func main() {
	const NCORES = 4
	const MAX = 10000000
	sendChannel := make(chan uint64, 100)
	returnChannel := make(chan bool, 100)
	var wg sync.WaitGroup
	wg.Add(NCORES)
	for i := 0; i < NCORES; i++ {
		go collatzWorker(sendChannel, returnChannel, &wg)
	}
	for i := 1; i < MAX; i++ {
		sendChannel <- uint64(i)
	}
	close(sendChannel)
	wg.Wait()
}

func collatzWorker(input chan uint64, output chan bool, wg *sync.WaitGroup) {
	for inp := range input {
		if !collatzWorks(inp) {
			fmt.Println("We got a bad one")
		}
		// output <- collatzWorks(inp)
	}
	wg.Done()
}

func collatzWorks(n uint64) bool {
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
