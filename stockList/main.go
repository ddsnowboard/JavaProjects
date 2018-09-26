package main

import "net/http"
import "fmt"
import "os"
import "regexp"

import "io/ioutil"

func main() {
	http.HandleFunc("/load/", func(w http.ResponseWriter, r *http.Request) {
            const URL string = "https://api.iextrading.com/1.0/stock/%s/price"
            URL_REGEX := regexp.MustCompile("/load/([A-Za-z]{1,5})/?$")
            matches := URL_REGEX.FindStringSubmatch(r.URL.Path)
            symbol := matches[1]
            resp, err := http.Get(fmt.Sprintf(URL, symbol))
            if err != nil {
                handleError(err)
            }
            defer resp.Body.Close()
            bytes, err := ioutil.ReadAll(resp.Body)
            if err != nil {
                handleError(err)
            }
            fmt.Fprintf(w, "%s", string(bytes))
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
