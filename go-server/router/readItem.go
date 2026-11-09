package router

import (
	"encoding/json"
	"go-server/database"
	types "go-server/types"
	"net/http"
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
