package router

import (
	"log"
	"net/http"
)

func InitServer() {
	// route handling goes here
	// http.Handle()
	// server opens the port here
	log.Println("See ya at http://127.0.0.1:8080")
	http.HandleFunc("/health", func(res http.ResponseWriter, req *http.Request) {
		res.WriteHeader(http.StatusOK)
		res.Header().Set("Content-Type", "application/json")
	})
	http.HandleFunc("/items", routeHandler)
	http.HandleFunc("/items/{id}", ReadOneItem) // 😄 ¯\_(ツ)_/¯ idk this guy below me. he is going thru things..
	// Todo: there is an extra 10ms latency per every switch case we add in route handler, this is stupid there should be a better way
	log.Fatal(http.ListenAndServe(":8080", nil))
	// ¯\_(ツ)_/¯
}

func routeHandler(response http.ResponseWriter, req *http.Request) {
	switch req.Method {
	case http.MethodGet:
		ReadAllItems(response, req)
	case http.MethodPost:
		CreateItemRequest(response, req)
	default:
		http.Error(response, `motherfucker who are you ¯\_(ツ)_/¯`, http.StatusMethodNotAllowed)
	}
}

//hola
