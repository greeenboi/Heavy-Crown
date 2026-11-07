package database

import (
	"database/sql"
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
