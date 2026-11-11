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
	var createError error
	var createdItem types.Item
	createQuery := `INSERT into items (name,description) VALUES (?, ?)`
	res, createError = db.Exec(createQuery, name, description)

	if createError != nil {
		return createdItem, createError
	}

	id, _ := res.LastInsertId()
	createdItem.Id = int(id)
	createdItem.Title = name
	createdItem.Description = description

	return createdItem, createError

}

func ReadAllItems() (items []types.Item, err error) {
	var res *sql.Rows
	var readError error
	var result []types.Item
	readAllQuery := `SELECT * from items`

	res, readError = db.Query(readAllQuery)

	if readError != nil {
		return result, readError
	}

	defer res.Close()

	for res.Next() {
		var item types.Item
		scanErr := res.Scan(&item.Id, &item.Title, &item.Description)
		if scanErr != nil {
			log.Println("nooo the error for reading an it is", scanErr)
			continue
		}
		result = append(result, item)
	}

	return result, readError
}

func ReadSingleItem(id int) (item types.Item, err error) {
	var res *sql.Rows
	var readError error
	var result types.Item

	readSingleQuery := `SELECT * from items WHERE id = ?`

	res, readError = db.Query(readSingleQuery, id)

	if readError != nil {
		return result, readError
	}

	defer res.Close()

	count := 0
	for res.Next() {
		var item types.Item
		scanErr := res.Scan(&item.Id, &item.Title, &item.Description)
		if scanErr != nil {
			log.Println("nooo the error for reading an it is", scanErr)
			return result, scanErr
		}
		result = item
		count += 1
	}

	if count == 0 {
		readError = errors.New("there is no id by that name")
		return result, readError
	}

	return result, readError
}
