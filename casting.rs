// Suppress all cast overflowwarmings
#![allow(overflowing_literals)]

fn main() {
	let dec = 65.432f32;

	// error! no implicit conersion
	//let int: u8 = decimal;
	let int = dec as u8;
	let character = int as char;
	println!("Casting: {} -> {} -> {}", dec, int, character);


	// when casting any value to an unsigned type, T,
	// std::T::MAX + 1 is added or subtracted until the value
	// fits into the new type

	// 1000 already fits in a u16
	println!("1000 as a u16 is: {}", 1000 as u16);

	// 1000 - 256 - 256 - 256 = 232
	// Under the hood, the first 8 least significant bits (LSB) are kept,
	// while the rest towards the most significant bit (MSB) get truncated.
	println!("1000 as a u8 is : {}", 1000 as u8);
	// -1 + 256 = 255
	println!("  -1 as a u8 is : {}", (-1i8) as u8);

	// For positive numbers, this is the same as the modulus
	println!("1000 mod 256 is : {}", 1000 % 256);

	// When casting to a signed type, the (bitwise) result is the same as
	// first casting to the corresponding unsigned type. If the most significant
	// bit of that value is 1, then the value is negative.

	// Unless it already fits, of course.
	println!(" 128 as a i16 is: {}", 128 as i16);
	// 128 as u8 -> 128, whose two's complement in eight bits is:
	println!(" 128 as a i8 is : {}", 128 as i8);

	// repeating the example above
	// 1000 as u8 -> 232
	println!("1000 as a u8 is : {}", 1000 as u8);
	// and the two's complement of 232 is -24
	println!(" 232 as a i8 is : {}", 232 as i8);

	// Suffixed literals, their types are known at initialization
	let x = 1u8;
	let y = 2u32;
	let z = 3f32;

	// Unsuffixed literal, their types depend on how they are used
	let i = 1;
	let f = 1.0;

	// `size_of_val` returns the size of a variable in bytes
	println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
	println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
	println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
	println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
	println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));


     // Because of the annotation, the compiler knows that `elem` has type u8.
	let elem = 5u8;

	// Create an empty vector (a growable array).
	let mut vec = Vec::new();
	// At this point the compiler doesn't know the exact type of `vec`, it
	// just knows that it's a vector of something (`Vec<_>`).

	// Insert `elem` in the vector.
	vec.push(elem);
	vec.push(elem);
	// Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
	// TODO ^ Try commenting out the `vec.push(elem)` line

	println!("{:?}", vec);



}

















