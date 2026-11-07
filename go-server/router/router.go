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
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func routeHandler(response http.ResponseWriter, req *http.Request) {
	switch req.Method {
	case http.MethodPost:
		CreateItemRequest(response, req)
	default:
		http.Error(response, "Method not allowed", http.StatusMethodNotAllowed)
	}
}

//hola
