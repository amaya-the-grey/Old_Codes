package main

import (
	"fmt"
	"math/rand"
)


func main() {
	// making array
	arr := make([]int, 10)

	// filling array
	for i := 0 ; i < cap(arr) ; i++ {
		arr[i] = rand.Intn(cap(arr))
	}
	fmt.Println("Array to sort: ", arr)
	// sorting array
	var min int
	var tmp int
	var idx int
	fmt.Println("Sorting...")
	for j := 0 ; j < cap(arr) ; j++ {
		min = arr[j]
		for i := j ; i < cap(arr) ; i++ {

			// finding minimum
			if arr[i] <= min {
				min = arr[i]
				idx = i
			}

		}
		// swapping
		tmp = arr[j]
		arr[j] = min
		arr[idx] = tmp
	}
	fmt.Println("Sorted array: ", arr)

}








