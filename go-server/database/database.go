package database

import (
	"database/sql"
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

}
