use cluFullTransmute::transmute_or_panic;
use core::fmt::Display;

/*
	For example, let's write some code with a Drop trait that panics when dropped and
	holds some data. We then transmute this data to another similar struct and check
	that we have effectively overridden the Drop trait and have a different struct
	with some data.

	We can also remove the Drop trait altogether or do any number of other things.
*/

/// Struct to panic when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PanicWhenDrop<T>(T);

impl<T> Drop for PanicWhenDrop<T> {
	fn drop(&mut self) {
		panic!("panic, discovered `drop(PanicWhenDrop);`");
	}
}

/// Struct to print value when dropped
#[derive(Debug)]
#[repr(transparent)]
struct PrintlnWhenDrop<T: Display>(T)
where
	T: Display;

impl<T> Drop for PrintlnWhenDrop<T>
where
	T: Display,
{
	fn drop(&mut self) {
		println!("println: {}", self.0);
	}
}

fn main() {
	let a: PanicWhenDrop<u16> = PanicWhenDrop(1024);
	println!("in a: {a:?}");

	let b: PrintlnWhenDrop<u16> = unsafe { transmute_or_panic(a as PanicWhenDrop<u16>) };
	println!("out b: {b:?}");

	drop(b); // <--- drop, PrintlnWhenDrop!
}
