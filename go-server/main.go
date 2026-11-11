// single file crud stuff,

package main

import (
	"go-server/database"
	"go-server/router"
)

// :D shh dw abt this
// func addition(val1 int, val2 int) (int, int) {
// 	var answer int
// 	answer = val1 + val2
// 	return answer, val1
// }

func main() {
	print("hello world \n")
	database.InitDB()

	defer database.Close()

	router.InitServer()
	// 1_ post , resp:{title,desc}

	//
}
