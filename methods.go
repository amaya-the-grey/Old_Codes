package main

import (
	"fmt"
	"math"
)

type Vertex struct {
	X, Y float64
}

// Abs function has reciever type Vertex with name v
// Abs is a method, a function with a reciever argument
func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

type MyFloat float64

func (f MyFloat) Abs_new() float64 {
	if f < 0 {
		return float64(-f)
	}
	return float64(f)
}

type I interface {
	M()
}

type T struct {
	S string
}

// This method means type T implements the interface I,
// but we don't need to explicitly declare that it does so.
func (t T) M() {
	fmt.Println(t.S)
}


func main() {
	// recievers
	v := Vertex{-3, 4}
	fmt.Println(v.Abs())
	f := MyFloat(-math.Sqrt2)
	fmt.Println(f.Abs_new())

	// pointer recievers
	v.Scale(10)
	fmt.Println(v.Abs())

	// interfaces are spooky 0.o
	var i I = T{"hello"}
	i.M()
	describe(i)

}
// note this reciever is a pointer!
func (v *Vertex) Scale(f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}

// interfaces are basically a tuple of (value, type)
func describe(i I) {
	fmt.Printf("(%v, %T)\n", i, i)
}
