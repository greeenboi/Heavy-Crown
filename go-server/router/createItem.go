package router

import (
	"encoding/json"
	"go-server/database"
	types "go-server/types"
	"net/http"
)

// create item and return it.
func CreateItemRequest(res http.ResponseWriter, req *http.Request) {
	//if method not post
	if req.Method != http.MethodPost {
		http.Error(res, "Invalid Method", http.StatusMethodNotAllowed)
		return
	}
	var item types.Item
	var err error
	var responseItem types.Item

	err = json.NewDecoder(req.Body).Decode(&item)

	if err != nil {
		http.Error(res, "invalid request body", http.StatusBadRequest)
		return
	}

	responseItem, err = database.InsertItem(item.Title, item.Description)

	if err != nil {
		http.Error(res, "internal server error", http.StatusInternalServerError)
		print("the db insert failed with", err)
		return
	}

	res.Header().Set("Content-Type", "application/json")
	json.NewEncoder(res).Encode(responseItem)

}
