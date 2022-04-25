
#![allow(unreachable_code)]
fn main() {
	let x = 5u32;

	let y = {
	    let x_squared = x * x;
	    let x_cube = x_squared * x;

	    // This expression will be assigned to `y`
	    x_cube + x_squared + x
	};

	let z = {
	    // The semicolon suppresses this expression and `()` is assigned to `z`
	    2 * x;
	};

	println!("x is {:?}", x);
	println!("y is {:?}", y);
	println!("z is {:?}", z);

	let n = 5;

	if n < 0 {
	    print!("{} is negative", n);
	} else if n > 0 {
	    print!("{} is positive", n);
	} else {
	    print!("{} is zero", n);
	}

	let big_n = 
	    if n < 10 && n > -10 {
		println!(", and is a small number, increase ten-fold");

		// This expression returns an `i32`.
		10 * n
	    } else {
		println!(", and is a big number, halve the number");

		// This expression must return an `i32` as well.
		n / 2
		// TODO ^ Try suppressing this expression with a semicolon.
	    };
	    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.
	
	println!("{} -> {}", n, big_n);



	// loops are for infinte loops, be sure to continue/break as needed
	let mut count = 0u32;

	println!("Let's count until infinity!");

	// Infinite loop
	loop {
	    count += 1;

	    if count == 3 {
		println!("three");

		// Skip the rest of this iteration
		continue;
	    }

	    println!("{}", count);

	    if count == 5 {
		println!("OK, that's enough");

		// Exit this loop
		break;
	    }
	}
	// you can nest loops too

    'outer: loop {
	println!("Entered the outer loop");

	'inner: loop {
	    println!("Entered the inner loop");

	    // This would break only the inner loop
	    //break;

	    // This breaks the outer loop
		break 'outer;
	}

	println!("This point will never be reached");
    }

    println!("Exited the outer loop");


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Counter: {}", result);

    // A counter variable
    let mut n = 1;

    // whiles are prety straight forward
    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // same exclusion on last val as python, 1..101 is just range()
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // alternatively, add inclusion on last val in range:
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    // can iterate through lists
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // allows list to be modified in place
    let mut names = vec!["Bob", "Frank", "Ferris"];
    println!("Before: {:?}", names);
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);



}









