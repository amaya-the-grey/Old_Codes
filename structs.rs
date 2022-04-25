#![allow(dead_code)]
#[derive(Debug)]

// note: allow dead code needs to go before derive debug
// as derive debug is apparently an "outer attribute"

struct Person<'a> {
	name: &'a str,
	age: u8,
}

struct Point {
	x: f32,
	y: f32,
}


enum WebEvent {
	PageLoad,
	PageUnload,
	KeyPress(char),
	Paste(String),
	Click {x:i64, y:i64},

}

// An attribute to hide warnings for unused code.

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}


// structs can be reused as fields of other structs
//#[allow(dead_code)]
struct Rectangle {
	p1: Point,
	p2: Point,
}

// to do: get this to pass the structure to the function and return
fn rect_area(rec: Rectangle) -> (f32) {
	let rarea = (rec.p1.x - rec.p2.x).abs()*(rec.p1.y - rec.p2.y).abs();
	rarea
}




fn main() {
	// create struct with field init shorthand
	let name = "Sydney";
	let age = 25;
	let syd = Person{ name, age };

	// only works because of debug print, otherwise can't
	println!("{:?}", syd);

	// need the x: and y: bits
	let point: Point = Point { x:0.3, y:0.4 };
	println!("Coords : {}, {}", point.x, point.y);


	let new_point = Point {x:0.5, y:0.6};
	println!("New Coords : {}, {}", new_point.x, new_point.y);

	let _rectangle = Rectangle {
		p2: Point {x: 0.5, y:0.6},
		p1: point,

	};
	let rarea = rect_area(_rectangle);
	println!("The rectangle with the above points has area {}", rarea);

	let pressed = WebEvent::KeyPress('x');
	// `to_owned()` creates an owned `String` from a string slice.
	let pasted  = WebEvent::Paste("my text".to_owned());
	let click   = WebEvent::Click { x: 20, y: 80 };
	let load    = WebEvent::PageLoad;
	let unload  = WebEvent::PageUnload;

	inspect(pressed);
	inspect(pasted);
	inspect(click);
	inspect(load);
	inspect(unload);

	// Explicitly `use` each name so they are available without
	// manual scoping.
	use crate::Status::{Poor, Rich};
	// Automatically `use` each name inside `Work`.
	use crate::Work::*;

	// Equivalent to `Status::Poor`.
	let status = Poor;
	// Equivalent to `Work::Civilian`.
	let work = Civilian;

	match status {
	    // Note the lack of scoping because of the explicit `use` above.
	    Rich => println!("The rich have lots of money!"),
	    Poor => println!("The poor have no money..."),
	}

	match work {
	    // Note again the lack of scoping.
	    Civilian => println!("Civilians work!"),
	    Soldier  => println!("Soldiers fight!"),
	}

	// `enums` can be cast as integers.
	println!("zero is {}", Number::Zero as i32);
	println!("one is {}", Number::One as i32);

	println!("roses are #{:06x}", Color::Red as i32);
	println!("violets are #{:06x}", Color::Blue as i32);

}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
	match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}



enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}






