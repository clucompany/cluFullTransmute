
use cluFullTransmute::mem::force_transmute;

/*
	This example is notable because by the Rust standard it does not allow 
	converting common types A to B, since it cannot check their sizes, 
	this example solves this.
	
	Additionally, as an example, we manipulate the Drop::drop function.
*/

/*
	----------------------------------------
	Why is repr(transparent) not being used?
	----------------------------------------
	Since this is an example, and we are working in the same program and 
	transmuting structures with the same data set, we may not use repr(transparent), 
	although this is not officially written anywhere :).
	
	So, if possible, use repr(transparent), but this is not always possible when there 
	are two or more data sets, of course you can additionally use repr(C)...
*/


struct A<T>(T);
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
	let data = A(9999usize); // We expect panic at the death of A.
	
	let b: B<usize> = unsafe { force_transmute(data) }; // type A no longer exists, it is now type B.
	
	assert_eq!(b.0, 9999usize); // Checking the value
	b.my_fn();
	
	drop(b);
	// That's it, no panic, type B.
}
