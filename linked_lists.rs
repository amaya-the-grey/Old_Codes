use crate::List::*;

enum List {
	// Cons: Tuple struct that wraps an element and a pointer to
	// the next node
	Cons(u32, Box<List>),
	// Nil: a node that signifies the end of the linked list
	Nil,

}


// Methods can be attached to an enum
impl List {
	// create an empty list
	fn new() -> List {
		// Nil has type 'List'
		Nil
	}
	// consume a list and return with new element at front
	fn prepend(self, elem: u32) -> List {
		// Cons also has type 'list'
		Cons(elem, Box::new(self))

	}

	// return length of list
	fn len(&self) -> u32 {
		// self has to be matched because the behavior depnds on it
		// self has type &List and *self has type list here
		// concrete type T is prefered to match of a reference &T

		match *self {
			Cons(_, ref tail) => 1 + tail.len(),
			Nil => 0

		}
	}

	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
			format!("{}, {}", head, tail.stringify())

			}
			Nil => {
			format!("Nil")
			},
		}
	}
}	
	
fn main() {

	let mut list = List::new();
	list = list.prepend(1);
	list = list.prepend(2);
	list = list.prepend(3);

	println!("Len : {}",list.len() );
	println!("{}", list.stringify());



}










