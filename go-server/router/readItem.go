package router

import (
	"encoding/json"
	"go-server/database"
	types "go-server/types"
	"net/http"
	"strconv"
)

func ReadAllItems(res http.ResponseWriter, req *http.Request) {
	if req.Method != http.MethodGet {
		http.Error(res, "Invalid Method", http.StatusMethodNotAllowed)
		return
	}

	var err error
	var itemList []types.Item

	itemList, err = database.ReadAllItems()

	if err != nil {
		http.Error(res, "internal server error", http.StatusInternalServerError)
		return
	}

	res.Header().Set("Content-Type", "application/json")
	json.NewEncoder(res).Encode(itemList)

}

func ReadOneItem(res http.ResponseWriter, req *http.Request) {
	if req.Method != http.MethodGet {
		http.Error(res, "Invalid Method", http.StatusMethodNotAllowed)
		return
	}

	id := req.PathValue("id")
	if id == "" {
		http.Error(res, "we need an id for this brother", http.StatusBadRequest)
		return
	}

	intId, idErr := strconv.Atoi(id)

	if idErr != nil {
		http.Error(res, "the id needs to be a int type chekc input", http.StatusBadRequest)
		return
	}

	var err error
	var item types.Item

	item, err = database.ReadSingleItem(intId)

	if err != nil {
		http.Error(res, err.Error(), http.StatusInternalServerError)
		return
	}

	res.Header().Set("Content-Type", "application/json")
	json.NewEncoder(res).Encode(item)

}
