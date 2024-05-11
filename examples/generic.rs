
use core::fmt::Display;
use cluFullTransmute::transmute_or_panic;

// Implementation of a simple transmutation with a generic parameter inside.

#[derive(Debug)]
#[repr(transparent)]
struct A<T> {
	#[allow(dead_code)]
	data: T
}

impl<T> Drop for A<T> {
	fn drop(&mut self) {
		panic!("Invalid beh");
	}
}

#[derive(Debug)]
#[repr(transparent)]
struct B<T> where T: Display {
	data: T,
}

impl<T> Drop for B<T> where T: Display {
	fn drop(&mut self) {
		println!("{}", self.data);
	}
}

fn main() {
	let a: A<u16> = A { // original and panic when falling
		data: 1024
	};
	println!("in: {:?}", a);
	
	let b: B<u16> = unsafe { transmute_or_panic(a) };
	println!("out: {:?}", b);
	
	drop(b); // <--- println!
}
