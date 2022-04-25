fn main() {
	let an_int = 1u32;
	let a_bool = true;
	let unit = ();

	// copy an int
	let copied_int = an_int;

	println!("an int {:?}", copied_int);
	println!("a bool {:?}", a_bool);
	println!("unt val {:?}", unit);


	let _unused_var = 3u32;
	//let noisy_unsued = 2u32;

	let _immutable_binding = 1;
	let mut mutable_binding = 1;

	println!("Before mutation: {}", mutable_binding);

	// Ok
	mutable_binding += 1;

	println!("After mutation: {}", mutable_binding);

	// Error!
	//_immutable_binding += 1;
	// FIXME ^ Comment out this line

	// This binding lives in the main function
	let long_lived_binding = 1;

	// This is a block, and has a smaller scope than the main function
	{
	    // This binding only exists in this block
	    let short_lived_binding = 2;

	    println!("inner short: {}", short_lived_binding);

	    // This binding *shadows* the outer one
	    let long_lived_binding = 5_f32;

	    println!("inner long: {}", long_lived_binding);
	}
	// End of the block

	// Error! `short_lived_binding` doesn't exist in this scope
	// println!("outer short: {}", short_lived_binding);
	// FIXME ^ Comment out this line

	println!("outer long: {}", long_lived_binding);
	
	// This binding also *shadows* the previous binding
	let long_lived_binding = 'a';
	
	println!("outer long: {}", long_lived_binding);




}




