package main

import (
	"fmt"
)


type Peg struct {
	enum int
	vals []int

}


func printTowers (A []int, B []int, C []int) {
	for i := cap(A) - 1; i >= 0; i-- {
		fmt.Printf("%v \t%v \t%v \n", A[i], B[i], C[i])
	}
}



func isFree(towerN int, nextPeg []int) bool {
/*
   Determines if you can move towerN to nextPeg. 
   Must follow the rule that a tower cannot sit on a smaller
   tower. 
*/
	// find top of stack on peg
	var top int
	top = stackTop(nextPeg)

	// check rule
	if nextPeg[top] == 0 {
		return true
	} else if towerN > nextPeg[top] {
		return false
	} else {
		return true
	}

}

func stackTop(pegVals []int) int {
/* Returns the index of the top of the peg tower */
	var top int
	for i := 0 ; i < cap(pegVals) ; i++ {
		if pegVals[i] == 0 { 
			top = i
			break 
		} else {
			top = cap(pegVals) - 1
		}
	}
	return top
}



func main() {
	// selecting random number of towers
	//n := rand.Intn(20)
	n := 5

	// making pegs
	pegA := Peg {
		enum: 0,
		vals: make([]int, n),
	}
	pegB := Peg {
		enum: 1,
		vals: make([]int, n),
	}
	pegC := Peg {
		enum: 2,
		vals: make([]int, n),
	}

	// filling peg A from bottom to top
	// initializing rest to zero
	for i := 0 ; i < n ; i++ {
		pegA.vals[i] = n - i
		pegB.vals[i] = 0
		pegC.vals[i] = 0
	}

	fmt.Println(pegA, pegB, pegC)
	fmt.Println(stackTop(pegA.vals), stackTop(pegB.vals))
	fmt.Println(isFree(pegA.vals[n-1], pegB.vals))
	printTowers(pegA.vals, pegB.vals, pegC.vals)



}








