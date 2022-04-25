package main

import (
	"fmt"
	"math/rand"
)


func main() {
	// making array
	arr := make([]int, 100)

	// filling array
	for i := 0 ; i < cap(arr) ; i++ {
		arr[i] = i + 1
	}

	// chosing random number to find
	num := rand.Intn(cap(arr))
	fmt.Printf("Finding number %v in array\n", num)

	max := cap(arr)
	min := 1
	guess := rand.Intn(max - min + 1)
	fmt.Println("initial guess: ", guess)
	counter := 0
	for num != guess{
		if num > guess {
			max = max
			min = guess
		} else {
			max = guess
			min = min
		}
		guess = min + rand.Intn(max - min + 1)
		counter = counter + 1
		fmt.Println("Guess: ", guess, "iteration: ", counter)
	}
	fmt.Printf("num found in %v iterations\n", counter)
}








