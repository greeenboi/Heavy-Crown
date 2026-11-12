package database

import (
	"database/sql"
	"errors"
	"go-server/types"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func InitDB() {
	var err error
	db, err = sql.Open("sqlite3", "./app.db")

	if err != nil {
		log.Fatal(err)
	}

	createQuery := `CREATE TABLE IF NOT EXISTS items(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL , description TEXT NOT NULL);`

	_, err = db.Exec(createQuery)
	if err != nil {
		log.Fatal(err)
	}

	println("yaay db running")
	// defer db.Close()
}

func Close() {
	db.Close()
	println("db session closed")
}

func InsertItem(name string, description string) (item types.Item, err error) {
	var res sql.Result
	// var err error
	// var createdItem types.Item
	createQuery := `INSERT into items (name,description) VALUES (?, ?)`
	res, err = db.Exec(createQuery, name, description)

	if err != nil {
		return item, err
	}

	id, _ := res.LastInsertId()
	item.Id = int(id)
	item.Title = name
	item.Description = description

	return item, err

}

func ReadAllItems() (items []types.Item, err error) {
	var res *sql.Rows
	// var err error
	// var items []types.Item
	readAllQuery := `SELECT * from items`

	res, err = db.Query(readAllQuery)

	if err != nil {
		return items, err
	}

	defer res.Close()

	for res.Next() {
		var item types.Item
		scanErr := res.Scan(&item.Id, &item.Title, &item.Description)
		if scanErr != nil {
			log.Println("nooo the error for reading an it is", scanErr)
			continue
		}
		items = append(items, item)
	}

	return items, err
}

func ReadSingleItem(id int) (item types.Item, err error) {
	var res *sql.Rows
	// var err error
	// var items types.Item

	readSingleQuery := `SELECT * from items WHERE id = ?`

	res, err = db.Query(readSingleQuery, id)

	if err != nil {
		return item, err
	}

	defer res.Close()

	count := 0
	for res.Next() {
		var scannedItem types.Item
		scanErr := res.Scan(&scannedItem.Id, &scannedItem.Title, &scannedItem.Description)
		if scanErr != nil {
			log.Println("nooo the error for reading an it is", scanErr)
			return item, scanErr
		}
		item = scannedItem
		count += 1
	}

	if count == 0 {
		err = errors.New("there is no id by that name")
		return item, err
	}

	return item, err
}
