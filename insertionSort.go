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
	fmt.Println("Sorting...")

	var idx int
	var val int
	for j := 0 ; j < cap(arr) ; j++ {
		val = arr[j]
		for i := 0 ; i <= j ; i++ {

			// finding position
			if val < arr[i] {
				idx = i
				// inserting
				insert(arr, idx, j)
				break
			}

		}
		fmt.Println(arr)
	}
	fmt.Println("Sorted array: ", arr)

}

func insert(arr []int, idx int, j int) {

	var tmp int
	var val int

	// inserting
	tmp = arr[idx]
	val = arr[j]
	arr[idx] = val
	val = tmp

	// shifting elements down
	for i := idx + 1 ; i <= j ; i++ {
	    tmp = arr[i]
	    arr[i] = val
	    val = tmp
	}

}



