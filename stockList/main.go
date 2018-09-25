package main

import "net/http"
import "fmt"
import "os"

func main() {
    http.HandleFunc("/load/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "%f", 42.88)
    })
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        http.ServeFile(w, r, "index.html")
    })
    err := http.ListenAndServe(":8080", nil)
    if err != nil {
        handleError(err)
    }
}

func handleError(err error) {
	fmt.Println("Got an error!")
	fmt.Println(err)
	os.Exit(1)
}
