// single file crud stuff,

package main

import "go-server/database"

func addition(val1 int, val2 int) (int, int) {
	var answer int
	answer = val1 + val2
	return answer, val1
}

func main() {
	print("hello world \n")
	x, y := addition(10, 20)
	println("addition is", x, "and y is", y)
	database.InitDB()
}
