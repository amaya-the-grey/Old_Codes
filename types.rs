// This is the main function
// compile via >> rustc types.rs
// then run executable

// prety much straight out of rust by example
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
	let bool = true;
	println!("Am I excited to start programming rust? {}", bool);

	println!("1 - 2 = {}", 1i32 - 2);
	// unsigned integer below doesn't work
	// println!("1 - 2 ?= {}", 1u32 - 2);
	
	// Short-circuiting boolean logic
	println!("true AND false is {}", true && false);
	println!("true OR false is {}", true || false);
	println!("NOT true is {}", !true);

	// Bitwise operations
	println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
	println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
	println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
	println!("1 << 5 is {}", 1u32 << 5);
	println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

	// Use underscores to improve readability!
	println!("One million is written as {}", 1_000_000u32);

	// A tuple with a bunch of different types
	let long_tuple = (1u8, 2u16, 3u32, 4u64,
			  -1i8, -2i16, -3i32, -4i64,
			  0.1f32, 0.2f64,
			  'a', true);

	// Values can be extracted from the tuple using tuple indexing
	println!("long tuple first value: {}", long_tuple.0);
	println!("long tuple second value: {}", long_tuple.1);

	// Tuples can be tuple members
	let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

	// Tuples are printable
	println!("tuple of tuples: {:?}", tuple_of_tuples);
	
	// But long Tuples cannot be printed
	// let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
	// println!("too long tuple: {:?}", too_long_tuple);
	// TODO ^ Uncomment the above 2 lines to see the compiler error

	let pair = (1, true);
	println!("pair is {:?}", pair);

	println!("the reversed pair is {:?}", reverse(pair));

	// Fixed-size array (type signature is superfluous)
	let xs: [i32; 5] = [1, 2, 3, 4, 5];

	// All elements can be initialized to the same value
	let ys: [i32; 500] = [0; 500];

	// Indexing starts at 0
	println!("first element of the array: {}", xs[0]);
	println!("second element of the array: {}", xs[1]);

	// `len` returns the size of the array
	println!("array size: {}", xs.len());

	// Arrays are stack allocated
	println!("array occupies {} bytes", mem::size_of_val(&xs));

	// Arrays can be automatically borrowed as slices
	println!("borrow the whole array as a slice");
	analyze_slice(&xs);

	// Slices can point to a section of an array
	println!("borrow a section of the array as a slice");
	analyze_slice(&ys[1 .. 4]);

	// Out of bound indexing causes compile error
	// but indentation is not an issue
    println!("{}", xs[4]);

}

