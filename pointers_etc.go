package main

import (
	"fmt"
	"strings"
	"math"
)

type Vertex struct {
	X int
	Y int
}

func printSlice(s []int) {
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)
}

type Coords struct {
	Lat, Long float64
}

var h map[string]Coords
var f = map[string]Coords{
	"Bell Labs": Coords{
		40.68433, -74.39967,
	},
	"Google": Coords{
		37.42202, -122.08408,
	},
}

func main() {
	// note: go has no pointer arithmetic
	i, j := 42, 2701

	p := &i         // point to i
	fmt.Println(*p) // read i through the pointer
	*p = 21         // set i through the pointer
	fmt.Println(i)  // see the new value of i

	p = &j         // point to j
	*p = *p / 37   // divide j through the pointer
	fmt.Println(j) // see the new value of j

	//strut them structs
	fmt.Println(Vertex{1, 2})

	// access struct fields using dots
	v := Vertex{1, 2}
	v.X = 4
	fmt.Println(v.X)

	// use pointers on structs
	q := &v
	q.X = 1e9
	fmt.Println(v)

	// arrays
	var a [2]string
	a[0] = "Hello"
	a[1] = "World"
	fmt.Println(a[0], a[1])
	fmt.Println(a)

	primes := [6]int{2, 3, 5, 7, 11, 13}
	fmt.Println(primes)

	// slices
	var l []int = primes[1:4]
	fmt.Println(l)

	names := [4]string{
		"John",
		"Paul",
		"George",
		"Ringo",
	}
	fmt.Println(names)

	c := names[0:2]
	b := names[1:3]
	fmt.Println(c, b)

	b[0] = "XXX"
	fmt.Println(c, b)
	fmt.Println(names)

	// slice literals
	m := []int{2, 3, 5, 7, 11, 13}
	fmt.Println(m)

	r := []bool{true, false, true, true, false, true}
	fmt.Println(r)

	n := []struct {
		i int
		b bool
	}{
		{2, true},
		{3, false},
		{5, true},
		{7, true},
		{11, false},
		{13, true},
	}
	fmt.Println(n)

	// slice defaults (like python)
	k := []int{2, 3, 5, 7, 11, 13}

	k = k[1:4]
	fmt.Println(k)

	k = k[:2]
	fmt.Println(k)

	k = k[1:]
	fmt.Println(k)

	// slice length and capacity
	s := []int{2, 3, 5, 7, 11, 13}
	printSlice(s)

	// Slice the slice to give it zero length.
	s = s[:0]
	printSlice(s)

	// Extend its length.
	s = s[:4]
	printSlice(s)

	// Drop its first two values.
	s = s[2:]
	printSlice(s)


	// empty slice value is "nil"
	var t []int
	fmt.Println(t, len(t), cap(t))
	if t == nil {
		fmt.Println("nil!")
	}


	// dynamically sized arrays wooooo!
	u := make([]int, 5)
	printSlice_new("u", u)

	e := make([]int, 0, 5)
	printSlice_new("e", e)

	w := e[:2]
	printSlice_new("w", w)

	z := w[2:5]
	printSlice_new("z", z)

	// Slices can include any type, even other slices
	// Create a tic-tac-toe board.
	board := [][]string{
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
		[]string{"_", "_", "_"},
	}

	// The players take turns.
	board[0][0] = "X"
	board[2][2] = "O"
	board[1][2] = "X"
	board[1][0] = "O"
	board[0][2] = "X"

	for i := 0; i < len(board); i++ {
		fmt.Printf("%s\n", strings.Join(board[i], " "))
	}

	// built in append fucn
	z = append(z, 2, 3, 4)
	printSlice(z)

	// range function returns both index of array, and copy of element
	var pow = []int{1, 2, 4, 8, 16, 32, 64, 128}
	// works because scope of variables i,v only in loop
	for i, v := range pow {
		fmt.Printf("2**%d = %d\n", i, v)
	}
	for _, value := range pow {
		fmt.Printf("%d\n", value)
	}

	// map is like dictonary with keys and values
	h = make(map[string]Coords)
	h["Bell Labs"] = Coords{
		40.68433, -74.39967,
	}
	fmt.Println(h["Bell Labs"])
	fmt.Println(f)

	mu := make(map[string]int)

	mu["Answer"] = 42
	fmt.Println("The value:", mu["Answer"])

	mu["Answer"] = 48
	fmt.Println("The value:", mu["Answer"])

	delete(mu, "Answer")
	fmt.Println("The value:", mu["Answer"])

	vi, ok := mu["Answer"]
	fmt.Println("The value:", vi, "Present?", ok)

	// functions are values and can be passed around as such
	hypot := func(x, y float64) float64 {
		return math.Sqrt(x*x + y*y)
	}
	fmt.Println(hypot(5, 12))

	fmt.Println(compute(hypot))
	fmt.Println(compute(math.Pow))

}

func printSlice_new(s string, x []int) {
	fmt.Printf("%s len=%d cap=%d %v\n",
		s, len(x), cap(x), x)
}


func compute(fn func(float64, float64) float64) float64 {
	return fn(3, 4)
}
