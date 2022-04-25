package main

import (
	"fmt"
	"math/rand"
)

func merge(arr *[]int, r int, q int, p int) {

	// create a temp array
	tmp := make([]int, p - r + 1)

	// go through arrays and in each iteration add smaller of both elements in temp 
	i := r
	j := q + 1
	k := 0
	for  i <= q {
		if j > p {
			break
		}
		if ((*arr)[i] <= (*arr)[j]) {
			tmp[k] = (*arr)[i]
			k = k + 1
			i = i + 1
		} else {
			tmp[k] = (*arr)[j]
			k = k + 1
			j = j + 1
		}

	}

	// add elements left in the first interval 
	for i <= q {
		tmp[k] = (*arr)[i]
		k = k + 1
		i = i + 1
	}

	// add elements left in the second interval 
	for j <= p {
		tmp[k] = (*arr)[j]
		k = k + 1
		j = j + 1
	}

	// copy temp to original array
	for i = r; i <= p; i++ {
		(*arr)[i] = tmp[i - r]
	}
}

func mergeSort(arr *[]int, r int, p int) {
	if(r < p) {
		q := (r + p) / 2;
		mergeSort(arr, r, q);
		mergeSort(arr, q + 1, p);
		merge(arr, r, q, p);
	}
}



func main() {
	// making array
	p := 19
	r := 0
	arr := make([]int, p + 1)

	// filling array
	for i := 0 ; i < cap(arr) ; i++ {
		arr[i] = rand.Intn(cap(arr))
	}
	fmt.Println("Array to sort: ", arr)
	// sorting array
	fmt.Println("Sorting...")
	mergeSort(&arr, r, p)
	fmt.Println("Sorted array: ", arr)

}








