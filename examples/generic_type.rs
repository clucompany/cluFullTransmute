
use cluFullTransmute::mem::force_transmute;

/*
	This example is notable because by the Rust standard it does not allow 
	converting common types A to B, since it cannot check their sizes, 
	this example solves this.
	
	Additionally, as an example, we manipulate the Drop::drop function.
*/

#[repr(transparent)]
struct A<T>(T);
#[repr(transparent)]
struct B<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

impl<T> Drop for B<T> {
	fn drop(&mut self) {
		
	}
}

fn main() {
	let a: A<usize> = A(9999usize); // We expect panic at the death of A.
	let b: B<usize> = unsafe { force_transmute(a) }; // type A no longer exists, it is now type B.
	
	assert_eq!(b.0, 9999usize); // Checking the value
	b.my_fn();
	
	drop(b);
	// That's it, no panic, type B.
}
