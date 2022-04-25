package main

import (
	"fmt"
	"time"
	"math/rand"
	"math"
	"math/cmplx"
)

var i, j int = 1, 2

var c, python, java = true, false, "no!"

var (
	ToBe   bool       = false
	MaxInt uint64     = 1<<64 - 1
	z      complex128 = cmplx.Sqrt(-5 + 12i)
)

const (
	// Create a huge number by shifting a 1 bit left 100 places.
	// In other words, the binary number that is 1 followed by 100 zeroes.
	Big = 1 << 100
	// Shift it right again 99 places, so we end up with 1<<1, or 2.
	Small = Big >> 99
)

func needInt(x int) int { return x*10 + 1 }
func needFloat(x float64) float64 {
	return x * 0.1
}

func add(x int, y int) int {
	return x + y
}

func swap(x, y string) (string, string) {
	return y, x
}
func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return x, y
}

/* 
Block comment life!
*/


func main() {
	// printing sandbox stuff

	// Print line
	fmt.Println("Hello, 世界")

	// time, rand, simple math
	fmt.Println("The time is", time.Now())
	fmt.Println("My favorite number is", rand.Intn(10), "not", rand.Intn(10))
	fmt.Println("Pi = ", math.Pi)

	// function calls
	fmt.Println("82 + 33 = ", add(82,33))
	a, b := swap("hello", "world")
	fmt.Println(a, b)
	fmt.Println(split(17))

	// var types and pretty printing
	var i int 
	fmt.Println(i, c, python, java)
	k := 3
	fmt.Println(k)

	// Printf
	fmt.Printf("Type: %T Value: %v\n", ToBe, ToBe)
	fmt.Printf("Type: %T Value: %v\n", MaxInt, MaxInt)
	fmt.Printf("Type: %T Value: %v\n", z, z)
	fmt.Printf("java: %q\n", java)

	// type conversions, explicit conversion required
	var x, y int = 3, 4
	var f float64 = math.Sqrt(float64(x*x + y*y))
	var z uint = uint(f)
	fmt.Println(x, y, z)

	// constants (besides death and taxes lol)
	const World = "世界"
	fmt.Println("Hello", World)
	const Truth = true
	fmt.Println("Go rules?", Truth)
	fmt.Println(needInt(Small))
	fmt.Println(needFloat(Small))
	fmt.Println(needFloat(Big))

}
