

use cluFullTransmute::mem::full_transmute;

#[allow(dead_code)]
struct A<T>(T);

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Strange behavior of the internal library.");
	}
}

#[allow(dead_code)]
struct B<T>(T);

impl<T> B<T> {
	pub fn my_fn(&self) {}
}

fn main() {
	let data = A(9999usize); //ignore drop!
	
	let b: B<usize> = unsafe{ full_transmute(data) };
	assert_eq!(b.0, 9999usize);
	
	b.my_fn();
}